use std::path::{Path, PathBuf};
use std::string::String;

const MAX_BRIGHTNESS_BASENAME: &str = "max_brightness";
const BRIGHTNESS_BASENAME: &str = "brightness";

// Defines the apparent path to the directory containing both
// the brightness and the max_brightness files.
const BRIGHTNESS_CONTAINING_DIR: &str = "/sys/class/backlight/amdgpu_bl0/";

// Reads the file named by |path| and returns the integral contents.
fn sysfs_file_to_int(path: &PathBuf) -> i32 {
    let contents = std::fs::read_to_string(path).unwrap();

    match contents.trim().parse::<i32>() {
        Ok(value) => return value,
        Err(why) => panic!("parse failure ({:#?}): {}", path, why),
    }
}

// Stringifies and writes |value| into file named by |path|.
fn write_int(path: &PathBuf, value: i32) {
    std::fs::write(path, value.to_string().as_bytes()).unwrap();
}

fn get_brightness_delta() -> i32 {
    let args: Vec<String> = std::env::args().collect();

    // We expect a integral value, like ``-312'' or ``520.''
    let delta_as_string = &args[1];
    let delta = delta_as_string.parse::<i32>();

    return delta.unwrap();
}

// Reads the current and max brightness; returns the new target
// brightness.
fn get_target_brightness(current: i32, max: i32) -> i32 {
    let tentative = current + get_brightness_delta();
    if tentative >= max {
        return max;
    } else if tentative < 0 {
        if current == 0 {
            return max;
        }
        return 0;
    }
    tentative
}

fn main() {
    let base_dir = Path::new(BRIGHTNESS_CONTAINING_DIR);
    let brightness_path: PathBuf = base_dir.join(BRIGHTNESS_BASENAME);

    let current = sysfs_file_to_int(&brightness_path);
    let max = sysfs_file_to_int(&base_dir.join(MAX_BRIGHTNESS_BASENAME));

    let new = get_target_brightness(current, max);
    write_int(&brightness_path, new);
}
