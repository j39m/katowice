use std::path::PathBuf;
use std::result::Result;

const LOOP_BACKING_DEVICE_BASENAME: &'static str = "flaglock6-home";

#[derive(Debug)]
enum AuricError {
    InvocationError(String),
    IoError(String),
}

struct SshfsManager {
    mountpoint: PathBuf,
}

impl SshfsManager {
    pub fn new() -> Result<SshfsManager, AuricError> {
        let xdg_runtime_dir = match std::env::var("XDG_RUNTIME_DIR") {
            Ok(val) => val,
            Err(e) => {
                return Err(AuricError::InvocationError(format!(
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
            .map_err(|e| AuricError::IoError(format!("bad mountpoint: ``{}''", e)))
    }

    pub fn loop_backing_device_path(&self) -> PathBuf {
        let mut result: PathBuf = self.mountpoint.clone();
        result.push(LOOP_BACKING_DEVICE_BASENAME);
        result
    }

    pub fn loop_backing_device_present(&self) -> bool {
        self.loop_backing_device_path().is_file()
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
