use regex::Regex;
use std::path::PathBuf;
use std::result::Result;
use subprocess::{Exec, ExitStatus, Redirection};

const LOOP_BACKING_DEVICE_BASENAME: &'static str = "flaglock6-home";
const LOOP_DEVICE_PARTITION_LABEL: &'static str = "fl6";
const RSYNC_DOT_NET_REMOTE_DIR: &'static str = "usw-s007.rsync.net:./";

const RUN_MEDIA_KALVIN: &'static str = "/run/media/kalvin";

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
    Rsync,
    LocalBackup,
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

struct LocalBackupManager {
    target: Regex,
}

struct AuricImpl {
    sshfs_manager: SshfsManager,
    loop_manager: LoopManager,
    luks_manager: LuksManager,
    local_backup_manager: LocalBackupManager,
}

fn exec_with_stdout(command: &str, args: &[&str], error_hint: &str) -> Result<String, AuricError> {
    let result = Exec::cmd(command)
        .args(args)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture()?;
    if !result.success() {
        return Err(AuricError::Subprocess(format!(
            "{} failed: ``{}''",
            error_hint,
            result.stderr_str(),
        )));
    }
    Ok(result.stdout_str())
}

fn exec(command: &str, args: &[&str], error_hint: &str) -> Result<(), AuricError> {
    exec_with_stdout(command, args, error_hint).map(|_stdout_str| ())
}

fn exec_rsync(args: &[&str]) -> Result<(), AuricError> {
    let result = Exec::cmd("rsync").args(args).join()?;
    if !result.success() {
        match result {
            ExitStatus::Exited(code) => {
                return Err(AuricError::Subprocess(format!(
                    "rsync exited with code {}",
                    code
                )))
            }
            ExitStatus::Signaled(signum) => {
                return Err(AuricError::Subprocess(format!(
                    "rsync exited with signal {}",
                    signum
                )))
            }
            ExitStatus::Other(code) => {
                return Err(AuricError::Subprocess(format!(
                    "unknown rsync error: {}",
                    code
                )))
            }
            ExitStatus::Undetermined => {
                return Err(AuricError::Subprocess("unknown rsync error!".to_string()))
            }
        }
    }
    Ok(())
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

        // Invoke `sshfs`
        // *    with compression,
        // *    allowing other users to access the mounted filesystem
        //      (necessary to allow loop-mounts), and
        // *    attempting to reconnect if we lose the connection.
        exec(
            "sshfs",
            &[
                "-C",
                "-o",
                "allow_other,reconnect",
                RSYNC_DOT_NET_REMOTE_DIR,
                self.mountpoint.to_str().unwrap(),
            ],
            "sshfs mount",
        )
    }

    pub fn unmount(&self) -> Result<(), AuricError> {
        if !self.loop_backing_device_present() {
            return Ok(());
        }
        exec(
            "fusermount",
            &["-u", self.mountpoint.to_str().unwrap()],
            "fusermount -u",
        )
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
        let losetup_stdout = exec_with_stdout("losetup", &["-l"], "loop device enumeration")?;
        for line in losetup_stdout.lines() {
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

        exec(
            "udisksctl",
            &["loop-setup", "-f", self.backing.to_str().unwrap()],
            "loop device attach",
        )
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
        exec(
            "udisksctl",
            &["loop-delete", "-b", attached_loop.to_str().unwrap()],
            "loop device detach",
        )
    }
}

impl LuksManager {
    pub fn new() -> LuksManager {
        let mut device = PathBuf::from("/dev/disk/by-label");
        device.push(LOOP_DEVICE_PARTITION_LABEL);
        LuksManager { device: device }
    }

    pub fn unlock(&self, locked_device: &PathBuf) -> Result<(), AuricError> {
        exec(
            "udisksctl",
            &["unlock", "-b", locked_device.to_str().unwrap()],
            format!("unlocking {}", locked_device.to_str().unwrap()).as_str(),
        )
    }

    pub fn lock(&self, underlying_device: &PathBuf) -> Result<(), AuricError> {
        exec(
            "udisksctl",
            &["lock", "-b", underlying_device.to_str().unwrap()],
            format!("locking {}", underlying_device.to_str().unwrap()).as_str(),
        )
    }

    pub fn mount(&self) -> Result<(), AuricError> {
        exec(
            "udisksctl",
            &["mount", "-b", self.device.to_str().unwrap()],
            format!("mounting {}", self.device.to_str().unwrap()).as_str(),
        )
    }

    pub fn unmount(&self) -> Result<(), AuricError> {
        exec(
            "udisksctl",
            &["unmount", "-b", self.device.to_str().unwrap()],
            format!("unmounting {}", self.device.to_str().unwrap()).as_str(),
        )
    }
}

impl LocalBackupManager {
    pub fn new() -> LocalBackupManager {
        LocalBackupManager {
            target: Regex::new(r"^\s+([\w-]+)\s+.+$").unwrap(),
        }
    }

    fn find_vg(&self) -> Result<String, AuricError> {
        exec(
            "udisksctl",
            &["unlock", "-b", "/dev/sda1"],
            "unlocking sda1",
        )?;

        let vgs_stdout = exec_with_stdout("sudo", &["vgs"], "VG enumeration")?;
        for line in vgs_stdout.lines() {
            if let Some(captures) = self.target.captures(line) {
                let vg_candidate = captures.get(1).unwrap().as_str();
                match vg_candidate {
                    "l-durey" | "g-tailleferre" => return Ok(String::from(vg_candidate)),
                    _ => (),
                }
            }
        }
        Err(AuricError::NotFound)
    }

    fn ready_lv_and_rsync(&self, lv: &str, lv_path: &str) -> Result<(), AuricError> {
        exec(
            "sudo",
            &["lvchange", "--monitor", "n", "-a", "y", lv],
            format!("readying LV ``{}''", lv).as_str(),
        )?;
        exec(
            "udisksctl",
            &["mount", "-b", lv_path],
            format!("mounting LV ``{}''", lv).as_str(),
        )?;
        if let Err(rsync_error) = exec_rsync(&[
            "--delete",
            "-avPS",
            "/home/kalvin/",
            format!("{}/j39m-home/", RUN_MEDIA_KALVIN).as_str(),
        ]) {
            eprintln!("rsync exited with error: {:#?}", rsync_error);
        }
        Ok(())
    }

    fn snapshot_lv(&self, lv: &str, lv_path: &str) -> Result<(), AuricError> {
        exec(
            "udisksctl",
            &["unmount", "-b", lv_path],
            format!("unmounting LV ``{}''", lv).as_str(),
        )?;
        if let Err(lvchange_error) = exec(
            "sudo",
            &["lvchange", "--monitor", "n", "-a", "n", lv],
            format!("un-readying LV ``{}''", lv).as_str(),
        ) {
            eprintln!("lvchange exited with error: {:#?}", lvchange_error);
        }
        let lv_snapshot_name = chrono::Local::today()
            .format("j39m-home-%Y-%m-%d")
            .to_string();
        exec(
            "sudo",
            &[
                "lvcreate",
                "--monitor",
                "n",
                "-s",
                "-n",
                lv_snapshot_name.as_str(),
                lv,
            ],
            format!("snapshotting LV ``{}''", lv).as_str(),
        )
    }

    fn teardown(&self, vg: &str) -> Result<(), AuricError> {
        if let Err(lvchange_error) = exec(
            "sudo",
            &["lvchange", "--monitor", "n", "-a", "n", vg],
            format!("un-readying VG ``{}''", vg).as_str(),
        ) {
            eprintln!("lvchange exited with error: {:#?}", lvchange_error);
        }
        exec("udisksctl", &["lock", "-b", "/dev/sda1"], "locking sda1")?;
        exec(
            "udisksctl",
            &["power-off", "-b", "/dev/sda"],
            "powering off sda",
        )
    }

    pub fn go(&self) -> Result<(), AuricError> {
        let vg = self.find_vg()?;
        let lv = format!("{}/j39m-home", vg.as_str());
        let lv_path = format!("/dev/{}", lv);
        self.ready_lv_and_rsync(lv.as_str(), lv_path.as_str())?;
        self.snapshot_lv(lv.as_str(), lv_path.as_str())?;
        self.teardown(vg.as_str())
    }
}

fn get_action() -> Result<AuricOperationMode, AuricError> {
    let mut args_iter = std::env::args();
    args_iter.next();
    if let Some(action) = args_iter.next() {
        match action.as_str() {
            "mount" => return Ok(AuricOperationMode::Mount),
            "unmount" => return Ok(AuricOperationMode::Unmount),
            "rsync" => return Ok(AuricOperationMode::Rsync),
            "lb" => return Ok(AuricOperationMode::LocalBackup),
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
            local_backup_manager: LocalBackupManager::new(),
        })
    }

    fn mount(&self) -> Result<(), AuricError> {
        println!("{}", "Mounting sshfs...");
        self.sshfs_manager.mount()?;
        println!("{}", "Attaching loop device...");
        self.loop_manager.attach()?;
        println!("{}", "Finding newly attached loop device...");
        let loop_device = self.loop_manager.find()?;
        println!("{}", "Unlocking volume...");
        self.luks_manager.unlock(&loop_device)?;
        println!("{}", "Mounting volume...");
        self.luks_manager.mount()
    }

    fn unmount(&self) -> Result<(), AuricError> {
        println!("{}", "Unmounting volume...");
        self.luks_manager.unmount()?;
        println!("{}", "Finding backing loop device...");
        let loop_device = self.loop_manager.find()?;
        println!("{}", "Locking volume...");
        self.luks_manager.lock(&loop_device)?;
        println!("{}", "Detaching loop device...");
        self.loop_manager.detach()?;
        println!("{}", "Unmounting sshfs...");
        self.sshfs_manager.unmount()
    }

    fn rsync(&self) -> Result<(), AuricError> {
        // I'm not pulling in the `home_dir` crate just to grab a value
        // that I suspect will be immutable for many years to come.
        // Likewise, the user mount directory is lazily hardcoded.
        let mut remote_directory = PathBuf::from(RUN_MEDIA_KALVIN);
        remote_directory.push(LOOP_DEVICE_PARTITION_LABEL);
        if !remote_directory.is_dir() {
            return Err(AuricError::Invocation(format!(
                "remote directory ``{}'' not mounted",
                remote_directory.display()
            )));
        }

        exec_rsync(&[
            "--delete",
            "-avPS",
            "--exclude=/.cache/",
            "--exclude=/.cargo/",
            "--exclude=/.local/share/",
            "--exclude=/.mozilla/",
            "--exclude=/.thunderbird/",
            "--exclude=/Downloads/.firefox-nightly/",
            "--exclude=/Downloads/.thunderbird-beta/",
            "--delete-excluded",
            "/home/kalvin/",
            remote_directory.to_str().unwrap(),
        ])
    }

    fn local_backup(&self) -> Result<(), AuricError> {
        self.local_backup_manager.go()
    }

    pub fn act(&self, mode: AuricOperationMode) -> Result<(), AuricError> {
        match mode {
            AuricOperationMode::Mount => return self.mount(),
            AuricOperationMode::Unmount => return self.unmount(),
            AuricOperationMode::Rsync => return self.rsync(),
            AuricOperationMode::LocalBackup => return self.local_backup(),
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
