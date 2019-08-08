use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::string::String;

const MAX_BRIGHTNESS_BASENAME: &str = "max_brightness";
const BRIGHTNESS_BASENAME: &str = "brightness";

// Defines the apparent path to the directory containing both
// the brightness and the max_brightness files.
const BRIGHTNESS_CONTAINING_DIR: &str = "/sys/class/backlight/intel_backlight/";

// Reads the file named by |path| and returns the integral contents.
fn sysfs_file_to_int(path: &PathBuf) -> std::result::Result<i32, Error> {
    let mut file = File::open(path)?;
    let mut raw_contents = String::new();
    file.read_to_string(&mut raw_contents)?;

    let contents = raw_contents.trim();

    match contents.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(why) => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("parse failure ({:#?}): {}", path, why),
        )),
    }
}

// Stringifies and writes |value| into file named by |path|.
fn write_int(path: &PathBuf, value: i32) -> std::io::Result<()> {
    let mut file = File::create(&path)?;
    file.write_all(&value.to_string().as_bytes())?;
    file.flush()
}

// Reads the current and max brightness; returns the new target
// brightness.
fn target_brightness(current: i32, max: i32) -> Result<i32, std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "missing delta arg"));
    }
    // We expect a integral value, like ``-312'' or ``520.''
    let delta_as_string = &args[1];
    let delta = delta_as_string.parse::<i32>();

    if !delta.is_ok() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("bad arg: {}", delta_as_string),
        ));
    }

    let tentative = current + delta.unwrap();
    if tentative >= max {
        return Ok(max);
    } else if tentative < 0 {
        // For kicks, set the brightness all the way up if caller is
        // trying to take it below 0.
        if current == 0 {
            return Ok(max);
        }
        return Ok(0);
    }
    Ok(tentative)
}

fn main() -> Result<(), std::io::Error> {
    let base_dir = Path::new(BRIGHTNESS_CONTAINING_DIR);
    let brightness_path: PathBuf = base_dir.join(BRIGHTNESS_BASENAME);

    let current = sysfs_file_to_int(&brightness_path)?;
    let max = sysfs_file_to_int(&base_dir.join(MAX_BRIGHTNESS_BASENAME))?;

    let new = target_brightness(current, max)?;
    write_int(&brightness_path, new)
}
