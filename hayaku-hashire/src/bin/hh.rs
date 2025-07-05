use std::os::unix::process::CommandExt;
use std::process::Command;

use hayaku_hashire::config::{CommandLine, Config};

fn get_config(target: &str) -> Config {
    let dirs = xdg::BaseDirectories::with_prefix("hayaku-hashire").unwrap();

    let mut toml_target = std::path::PathBuf::from(target);
    toml_target.set_extension("toml");

    let target_path = dirs.find_config_file(toml_target).unwrap();
    let utf8 = std::fs::read(target_path).unwrap();
    let contents = String::from_utf8_lossy(&utf8);

    toml::from_str(&contents).unwrap()
}

fn get_command() -> Command {
    let mut args_less_the_first = std::env::args();
    // Pops the name of this executable.
    args_less_the_first.next();
    // Pops the name of the hh target.
    let target = args_less_the_first.next().unwrap();
    // Copies and collects the remaining argv.
    let args: Vec<String> = args_less_the_first.collect();

    let mut config = get_config(&target).as_args().unwrap();
    config.extend(args);

    let mut command: Command = Command::new(&config[0]);
    command.args(&config[1..]);

    command
}

fn main() {
    let mut command = get_command();
    if cfg!(debug_assertions) {
        eprintln!("{:#?}", command);
    }
    let _ = command.exec();
}
