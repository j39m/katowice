use regex::Regex;
use std::path::PathBuf;
use std::result::Result;
use subprocess::{Exec, Redirection};

const LOOP_BACKING_DEVICE_BASENAME: &'static str = "flaglock6-home";
const LOOP_DEVICE_PARTITION_LABEL: &'static str = "fl6";

#[derive(Debug)]
pub enum AuricError {
    Invocation(String),
    Io(String),
    Subprocess(String),
    NotFound,
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

    pub fn ensure_mountpoint_is_dir(&self) -> Result<(), AuricError> {
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

    pub fn loop_backing_device_present(&self) -> bool {
        self.loop_backing_device_path().is_file()
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
        let result = Exec::cmd("sudo")
            .arg("losetup")
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

fn main_impl() -> Result<(), AuricError> {
    println!("Hello there!");
    Ok(())
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
