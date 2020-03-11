use std::os::unix::process::CommandExt;
use std::process::Command;

fn simple_firejail_command(target: &str, args: std::env::Args) -> Command {
    let mut command = Command::new("/usr/bin/firejail");

    command.arg(format!("--profile=/etc/firejail/{}.profile", target));
    command.arg(format!("/usr/bin/{}", target));

    let argv_remainder : Vec<String> = args.collect();
    command.args(argv_remainder);

    command
}

fn init_command() -> Command {
    let mut args = std::env::args();
    args.next();

    let target = args.next().unwrap();
    match target.as_str() {
        "npv" => return simple_firejail_command("mpv", args),
        "vlk" => return simple_firejail_command("vlc", args),
        "z" => return simple_firejail_command("zathura", args),
        _ => panic!(format!("no handler for ``{}''", target)),
    }
}

fn main() {
    let mut command = init_command();
    command.exec();
}
