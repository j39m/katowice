use std::os::unix::process::CommandExt;
use std::process::Command;

const SYSTEMD_RUN: &'static str = "/usr/bin/systemd-run";
const FIREJAIL: &'static str = "/usr/bin/firejail";

const FIREFOX: &'static str = "/usr/bin/firefox";
const FIREFOX_PROFILE: &'static str = "/etc/firejail/firefox.profile";
const FIREFOX_MOZILLA_SFW_PROFILE: &'static str = "default-1473025815439";

const FIREFOX_MEMORY_HIGH: i32 = 4420;
const FIREFOX_MEMORY_MAX: i32 = 5200;

const KEIRA: &'static str = "/home/kalvin/Downloads/.firefox-nightly/firefox";
const KEIRA_PROFILE: &'static str = "/etc/firejail/firefox-nightly.profile";

const TERM: &'static str = "/usr/bin/alacritty";
const TERM_PROFILE: &'static str = "/etc/firejail/x-terminal-emulator.profile";

const THUNDERBIRD: &'static str = "/home/kalvin/Downloads/.thunderbird-beta/thunderbird";
const THUNDERBIRD_PROFILE: &'static str = "/etc/firejail/thunderbird-beta.profile";

#[derive(Debug)]
struct CgroupedFirejailedCommandOptions<'a> {
    bin_path: &'a str,
    memory_high: Option<i32>,
    memory_max: Option<i32>,
    firejail_profile: Option<&'a str>,
    implicit_extra_args: Option<&'a [&'a str]>,
    argv_remainder: Vec<String>,
}

fn cgrouped_firejail_command(options: CgroupedFirejailedCommandOptions) -> Command {
    let mut command = Command::new(SYSTEMD_RUN);
    command.args(&["--user", "--scope"]);
    command.args(&["-p", "MemorySwapMax=0"]);

    if let Some(memory_high) = options.memory_high {
        let memory_high_owned_string = format!("MemoryHigh={}M", memory_high);
        command.args(&["-p", memory_high_owned_string.as_str()]);
    }
    if let Some(memory_max) = options.memory_max {
        let memory_max_owned_string = format!("MemoryMax={}M", memory_max);
        command.args(&["-p", memory_max_owned_string.as_str()]);
    }

    if let Some(firejail_profile) = options.firejail_profile {
        command.arg(FIREJAIL);
        command.arg("--ignore=seccomp");
        command.arg(format!("--profile={}", firejail_profile));
    }

    command.arg(options.bin_path);

    if let Some(implicit_extra_args) = options.implicit_extra_args {
        command.args(implicit_extra_args);
    }

    command.args(options.argv_remainder);

    command
}

fn simple_firejail_command(target: &str, args: Vec<String>) -> Command {
    let mut command = Command::new(FIREJAIL);

    command.arg(format!("--profile=/etc/firejail/{}.profile", target));
    command.arg(format!("/usr/bin/{}", target));
    command.args(args);

    command
}

fn init_command() -> Command {
    let mut args_less_the_first = std::env::args();
    // Pops the name of this executable.
    args_less_the_first.next();

    // Pops the name of the unner target.
    let target = args_less_the_first.next().unwrap();

    // Copies and collects the remaining argv.
    let args: Vec<String> = args_less_the_first.collect();

    match target.as_str() {
        "ff" => {
            return cgrouped_firejail_command(CgroupedFirejailedCommandOptions {
                bin_path: FIREFOX,
                memory_high: Some(FIREFOX_MEMORY_HIGH),
                memory_max: Some(FIREFOX_MEMORY_MAX),
                firejail_profile: Some(FIREFOX_PROFILE),
                implicit_extra_args: Some(&["-P", FIREFOX_MOZILLA_SFW_PROFILE]),
                argv_remainder: args,
            });
        }
        "keira" => {
            return cgrouped_firejail_command(CgroupedFirejailedCommandOptions {
                bin_path: KEIRA,
                memory_high: Some(FIREFOX_MEMORY_HIGH),
                memory_max: Some(FIREFOX_MEMORY_MAX),
                firejail_profile: Some(KEIRA_PROFILE),
                implicit_extra_args: Some(&["-P", "nightly"]),
                argv_remainder: args,
            });
        }
        "npv" => return simple_firejail_command("mpv", args),
        "t" => {
            return cgrouped_firejail_command(CgroupedFirejailedCommandOptions {
                bin_path: TERM,
                memory_high: None,
                memory_max: None,
                firejail_profile: Some(TERM_PROFILE),
                implicit_extra_args: None,
                argv_remainder: args,
            })
        }
        "read" => {
            let formatted_args: Vec<String> = vec![format!("about:reader?url={}", args[0])];
            return cgrouped_firejail_command(CgroupedFirejailedCommandOptions {
                bin_path: FIREFOX,
                memory_high: Some(FIREFOX_MEMORY_HIGH),
                memory_max: Some(FIREFOX_MEMORY_MAX),
                firejail_profile: Some(FIREFOX_PROFILE),
                implicit_extra_args: Some(&["-P", FIREFOX_MOZILLA_SFW_PROFILE]),
                argv_remainder: formatted_args,
            });
        }
        "tbb" => {
            return cgrouped_firejail_command(CgroupedFirejailedCommandOptions {
                bin_path: THUNDERBIRD,
                memory_high: None,
                memory_max: None,
                firejail_profile: Some(THUNDERBIRD_PROFILE),
                implicit_extra_args: None,
                argv_remainder: args,
            });
        }
        "vlk" => {
            return cgrouped_firejail_command(CgroupedFirejailedCommandOptions {
                bin_path: "/usr/bin/vlc",
                memory_high: None,
                memory_max: None,
                firejail_profile: Some("/etc/firejail/vlc.profile"),
                implicit_extra_args: None,
                argv_remainder: args,
            })
        }
        "z" => return simple_firejail_command("zathura", args),
        _ => panic!(format!("no handler for ``{}''", target)),
    }
}

fn main() {
    let mut command = init_command();
    println!("{:#?}", command);
    command.exec();
}
