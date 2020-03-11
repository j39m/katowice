use std::os::unix::process::CommandExt;
use std::process::Command;

fn simple_firejail_command(target: &str, args: std::env::Args) -> Command {
    let mut command = Command::new("/usr/bin/firejail");

    let executable_target = match target {
        "npv" => "mpv",
        "vlk" => "vlc",
        "z" => "zathura",
        _ => panic!("BUG: unhandled executable_target"),
    };
    command.arg(format!("--profile=/etc/firejail/{}.profile", executable_target));
    command.arg(format!("/usr/bin/{}", executable_target));

    let argv_remainder : Vec<String> = args.collect();
    command.args(argv_remainder);

    command
}

fn init_command() -> Command {
    let mut args = std::env::args();
    args.next();

    let target = args.next().unwrap();
    match target.as_str() {
        "npv" | "vlk" | "z" => return simple_firejail_command(&target, args),
        _ => panic!(format!("no handler for ``{}''", target)),
    }
}

fn main() {
    let mut command = init_command();
    command.exec();
}
