use regex::Regex;
use std::path::PathBuf;
use std::result::Result;
use subprocess::{Exec, Redirection};

const LOOP_BACKING_DEVICE_BASENAME: &'static str = "flaglock6-home";
const LOOP_DEVICE_PARTITION_LABEL: &'static str = "fl6";
const RSYNC_DOT_NET_REMOTE_DIR: &'static str = "usw-s007.rsync.net:./";

#[derive(Debug)]
pub enum AuricError {
    Invocation(String),
    Io(String),
    Subprocess(String),
    NotFound,
}

enum AuricOperationMode {
    Mount,
    Unmount,
}

impl From<subprocess::PopenError> for AuricError {
    fn from(err: subprocess::PopenError) -> AuricError {
        AuricError::Subprocess(err.to_string())
    }
}

struct SshfsManager {
    mountpoint: PathBuf,
}

struct LoopManager {
    target: Regex,
    backing: PathBuf,
}

struct LuksManager {
    device: PathBuf,
}

struct AuricImpl {
    sshfs_manager: SshfsManager,
    loop_manager: LoopManager,
    luks_manager: LuksManager,
}

impl SshfsManager {
    pub fn new() -> Result<SshfsManager, AuricError> {
        let xdg_runtime_dir = match std::env::var("XDG_RUNTIME_DIR") {
            Ok(val) => val,
            Err(e) => {
                return Err(AuricError::Invocation(format!(
                    "fatal error getting ${{XDG_RUNTIME_DIR}}: ``{}''",
                    e
                )))
            }
        };
        let mut mountpoint = PathBuf::new();
        mountpoint.push(xdg_runtime_dir);
        mountpoint.push("rsync.net-mountpoint");

        Ok(SshfsManager {
            mountpoint: mountpoint,
        })
    }

    fn ensure_mountpoint_is_dir(&self) -> Result<(), AuricError> {
        if self.mountpoint.is_dir() {
            return Ok(());
        }
        std::fs::create_dir(self.mountpoint.as_path())
            .map_err(|e| AuricError::Io(format!("bad mountpoint: ``{}''", e)))
    }

    // Does _not_ ensure that backing device exists.
    pub fn loop_backing_device_path(&self) -> PathBuf {
        let mut result: PathBuf = self.mountpoint.clone();
        result.push(LOOP_BACKING_DEVICE_BASENAME);
        result
    }

    fn loop_backing_device_present(&self) -> bool {
        self.loop_backing_device_path().is_file()
    }

    pub fn mount(&self) -> Result<(), AuricError> {
        self.ensure_mountpoint_is_dir()?;
        if self.loop_backing_device_present() {
            return Ok(());
        }

        let result = Exec::cmd("sshfs")
            .arg("-o")
            .arg("allow_other")
            .arg(RSYNC_DOT_NET_REMOTE_DIR)
            .arg(&self.mountpoint)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .capture()?;
        if !result.success() {
            return Err(AuricError::Subprocess(format!(
                "sshfs failed: ``{}''",
                result.stderr_str()
            )));
        }
        Ok(())
    }

    pub fn unmount(&self) -> Result<(), AuricError> {
        if !self.loop_backing_device_present() {
            return Ok(());
        }

        let result = Exec::cmd("fusermount")
            .arg("-u")
            .arg(&self.mountpoint)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .capture()?;
        if !result.success() {
            return Err(AuricError::Subprocess(format!(
                "fusermount -u failed: ``{}''",
                result.stderr_str()
            )));
        }
        Ok(())
    }
}

impl LoopManager {
    pub fn new(backing: &PathBuf) -> LoopManager {
        let target = format!(
            r"^(/dev/loop\d)(\s+\d+){{4}}\s+({})\s+",
            backing.to_str().unwrap()
        );
        LoopManager {
            target: Regex::new(&target).unwrap(),
            backing: backing.clone(),
        }
    }

    pub fn find(&self) -> Result<PathBuf, AuricError> {
        let result = Exec::cmd("losetup")
            .arg("-l")
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .capture()?;
        if !result.success() {
            return Err(AuricError::Subprocess(format!(
                "failed to list loop devices: ``{}''",
                result.stderr_str()
            )));
        }

        for line in result.stdout_str().lines() {
            if let Some(captures) = self.target.captures(line) {
                return Ok(PathBuf::from(captures.get(1).unwrap().as_str()));
            }
        }
        Err(AuricError::NotFound)
    }

    pub fn attach(&self) -> Result<(), AuricError> {
        if let Ok(_attached_loop) = self.find() {
            return Ok(());
        }

        let result = Exec::cmd("udisksctl")
            .arg("loop-setup")
            .arg("-f")
            .arg(&self.backing)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .capture()?;
        if !result.success() {
            return Err(AuricError::Subprocess(format!(
                "failed to attach loop device: ``{}''",
                result.stderr_str()
            )));
        }
        Ok(())
    }

    // Note that the loop device will linger if the volume remains
    // unlocked. Prefer not to call this before locking.
    pub fn detach(&self) -> Result<(), AuricError> {
        let attached_loop = match self.find() {
            Ok(path) => path,
            Err(e) => {
                if let AuricError::NotFound = e {
                    return Ok(());
                }
                return Err(e);
            }
        };
        let result = Exec::cmd("udisksctl")
            .arg("loop-delete")
            .arg("-b")
            .arg(&attached_loop)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .capture()?;
        if !result.success() {
            return Err(AuricError::Subprocess(format!(
                "failed to detach loop device: ``{}''",
                result.stderr_str()
            )));
        }
        Ok(())
    }
}

impl LuksManager {
    pub fn new() -> LuksManager {
        let mut device = PathBuf::from("/dev/disk/by-label");
        device.push(LOOP_DEVICE_PARTITION_LABEL);
        LuksManager { device: device }
    }

    // `locked_device` is necessary as an external argument because no
    // device mapping exists in a pre-unlock world, so we need to target
    // the loop device directly.
    pub fn unlock(&self, locked_device: &PathBuf) -> Result<(), AuricError> {
        let result = Exec::cmd("udisksctl")
            .arg("unlock")
            .arg("-b")
            .arg(locked_device)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .capture()?;
        if !result.success() {
            return Err(AuricError::Subprocess(format!(
                "failed to unlock {}: ``{}''",
                locked_device.to_str().unwrap(),
                result.stderr_str()
            )));
        }
        Ok(())
    }

    // No extra argument is needed here (c.f. `self::unlock()`) because
    // we can go straight after the well-known device available in
    // /dev/disk/by-label.
    pub fn lock(&self) -> Result<(), AuricError> {
        let result = Exec::cmd("udisksctl")
            .arg("lock")
            .arg("-b")
            .arg(&self.device)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .capture()?;
        if !result.success() {
            return Err(AuricError::Subprocess(format!(
                "failed to lock {}: ``{}''",
                self.device.to_str().unwrap(),
                result.stderr_str()
            )));
        }
        Ok(())
    }

    pub fn mount(&self) -> Result<(), AuricError> {
        let result = Exec::cmd("udisksctl")
            .arg("mount")
            .arg("-b")
            .arg(&self.device)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .capture()?;
        if !result.success() {
            return Err(AuricError::Subprocess(format!(
                "failed to mount {}: ``{}''",
                self.device.to_str().unwrap(),
                result.stderr_str()
            )));
        }
        Ok(())
    }

    pub fn unmount(&self) -> Result<(), AuricError> {
        let result = Exec::cmd("udisksctl")
            .arg("unmount")
            .arg("-b")
            .arg(&self.device)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Pipe)
            .capture()?;
        if !result.success() {
            return Err(AuricError::Subprocess(format!(
                "failed to unmount {}: ``{}''",
                self.device.to_str().unwrap(),
                result.stderr_str()
            )));
        }
        Ok(())
    }
}

fn get_action() -> Result<AuricOperationMode, AuricError> {
    let mut args_iter = std::env::args();
    args_iter.next();
    if let Some(action) = args_iter.next() {
        match action.as_str() {
            "mount" => return Ok(AuricOperationMode::Mount),
            "unmount" => return Ok(AuricOperationMode::Unmount),
            _ => {
                return Err(AuricError::Invocation(format!(
                    "unknown action: ``{}''",
                    action
                )))
            }
        }
    }
    Err(AuricError::Invocation("no action given".to_string()))
}

impl AuricImpl {
    pub fn new() -> Result<AuricImpl, AuricError> {
        let sshfs_manager = SshfsManager::new()?;
        let loop_backing_device_path = sshfs_manager.loop_backing_device_path();
        Ok(AuricImpl {
            sshfs_manager: sshfs_manager,
            loop_manager: LoopManager::new(&loop_backing_device_path),
            luks_manager: LuksManager::new(),
        })
    }

    fn mount(&self) -> Result<(), AuricError> {
        self.sshfs_manager.mount()?;
        self.loop_manager.attach()?;
        let loop_device = self.loop_manager.find()?;
        self.luks_manager.unlock(&loop_device)?;
        self.luks_manager.mount()
    }

    fn unmount(&self) -> Result<(), AuricError> {
        self.luks_manager.unmount()?;
        self.luks_manager.lock()?;
        self.loop_manager.detach()?;
        self.sshfs_manager.unmount()
    }

    pub fn act(&self, mode: AuricOperationMode) -> Result<(), AuricError> {
        match mode {
            AuricOperationMode::Mount => return self.mount(),
            AuricOperationMode::Unmount => return self.unmount(),
        }
    }
}

fn main_impl() -> Result<(), AuricError> {
    let action = get_action()?;
    let auric_impl = AuricImpl::new()?;
    auric_impl.act(action)
}

fn main() {
    std::process::exit(match main_impl() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{:?}", e);
            1
        }
    });
}
