use std::io::Write;
use std::option::Option;
use std::os::unix::process::CommandExt;
use std::process::Command;

const FIREFOX: &'static str = "/usr/bin/firefox";
const FIREFOX_MOZILLA_SFW_PROFILE: &'static str = "default-1473025815439";

const FIREFOX_MEMORY_HIGH: i32 = 9100;
const FIREFOX_MEMORY_MAX: i32 = 10400;

// Partial ordering on enums are ordered by their declaration order:
// https://users.rust-lang.org/t/how-to-sort-enum-variants/52291/2
#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum BuilderWatermark {
    Unset,
    MemoryHigh,
    MemoryMax,
    FirejailProfile,
    BinPath,
    ImplicitExtraArgs,
    RemainingArgs,
}

struct CgroupedFirejailedCommand {
    pub command: std::process::Command,
    watermark: BuilderWatermark,
}

impl CgroupedFirejailedCommand {
    pub fn new() -> Self {
        let mut result = Self {
            command: std::process::Command::new("/usr/bin/systemd-run"),
            watermark: BuilderWatermark::Unset,
        };
        result
            .command
            .args(&["--user", "--scope", "-p", "MemorySwapMax=0"]);
        result
    }

    pub fn memory_high(mut self, param: i32) -> Self {
        assert!(self.watermark < BuilderWatermark::MemoryHigh);
        self.watermark = BuilderWatermark::MemoryHigh;
        self.command.arg("-p");
        self.command.arg(format!("MemoryHigh={}M", param));
        self
    }

    pub fn memory_max(mut self, param: i32) -> Self {
        assert!(self.watermark < BuilderWatermark::MemoryMax);
        self.watermark = BuilderWatermark::MemoryMax;
        self.command.arg("-p");
        self.command.arg(format!("MemoryMax={}M", param));
        self
    }

    pub fn firejail_profile(mut self, path: &str) -> Self {
        assert!(self.watermark < BuilderWatermark::FirejailProfile);
        self.watermark = BuilderWatermark::FirejailProfile;
        self.command
            .args(&["/usr/bin/firejail", "--ignore=seccomp"]);
        self.command
            .arg(format!("--profile=/etc/firejail/{}.profile", path));
        self
    }

    pub fn bin_path(mut self, path: &str) -> Self {
        assert!(self.watermark < BuilderWatermark::BinPath);
        self.watermark = BuilderWatermark::BinPath;
        self.command.arg(path);
        self
    }

    pub fn implicit_extra_args(mut self, args: &[&str]) -> Self {
        assert!(self.watermark < BuilderWatermark::ImplicitExtraArgs);
        self.watermark = BuilderWatermark::ImplicitExtraArgs;
        self.command.args(args);
        self
    }

    pub fn remaining_args(mut self, args: Vec<String>) -> Self {
        assert!(self.watermark < BuilderWatermark::RemainingArgs);
        self.watermark = BuilderWatermark::RemainingArgs;
        self.command.args(args);
        self
    }
}

fn simple_firejail_command(target: &str, args: Vec<String>) -> Command {
    let mut command = Command::new("/usr/bin/firejail");

    command.arg(format!("--profile=/etc/firejail/{}.profile", target));
    command.arg(format!("/usr/bin/{}", target));
    command.args(args);

    command
}

fn quodlibet_command(args: Vec<String>) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("/home/kalvin/.config/quodlibet/control")
        .unwrap();
    file.write_all(match args[0].as_str() {
        "pp" => "play-pause".as_bytes(),
        "sa" => "stop-after 1".as_bytes(),
        _ => panic!(
            "bad argument for quodlibet command: ``{}''",
            args[0].as_str()
        ),
    })
    .unwrap();
}

fn init_command() -> Option<Command> {
    let mut args_less_the_first = std::env::args();
    // Pops the name of this executable.
    args_less_the_first.next();

    // Pops the name of the unner target.
    let target = args_less_the_first.next().unwrap();

    // Copies and collects the remaining argv.
    let args: Vec<String> = args_less_the_first.collect();

    match target.as_str() {
        "ef" => {
            // edgy Firefox; Firejail disabled
            return Some(
                CgroupedFirejailedCommand::new()
                    .memory_high(FIREFOX_MEMORY_HIGH)
                    .memory_max(FIREFOX_MEMORY_MAX)
                    .bin_path(FIREFOX)
                    .implicit_extra_args(&["-P", FIREFOX_MOZILLA_SFW_PROFILE])
                    .remaining_args(args)
                    .command,
            );
        }
        "ff" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .memory_high(FIREFOX_MEMORY_HIGH)
                    .memory_max(FIREFOX_MEMORY_MAX)
                    .firejail_profile("firefox")
                    .bin_path(FIREFOX)
                    .implicit_extra_args(&["-P", FIREFOX_MOZILLA_SFW_PROFILE])
                    .remaining_args(args)
                    .command,
            );
        }
        "keira" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .memory_high(FIREFOX_MEMORY_HIGH)
                    .memory_max(FIREFOX_MEMORY_MAX)
                    .firejail_profile("firefox-nightly")
                    .bin_path("/home/kalvin/Downloads/.firefox-nightly/firefox")
                    .implicit_extra_args(&["-P", "nightly"])
                    .remaining_args(args)
                    .command,
            );
        }
        "npv" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .firejail_profile("mpv")
                    .bin_path("/usr/bin/mpv")
                    .remaining_args(args)
                    .command,
            );
        }
        "q" => {
            quodlibet_command(args);
            return None;
        }
        "read" => {
            let formatted_args: Vec<String> = vec![format!("about:reader?url={}", args[0])];
            return Some(
                CgroupedFirejailedCommand::new()
                    .memory_high(FIREFOX_MEMORY_HIGH)
                    .memory_max(FIREFOX_MEMORY_MAX)
                    .firejail_profile("firefox")
                    .bin_path(FIREFOX)
                    .implicit_extra_args(&["-P", FIREFOX_MOZILLA_SFW_PROFILE])
                    .remaining_args(formatted_args)
                    .command,
            );
        }
        "t" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .firejail_profile("x-terminal-emulator")
                    .bin_path("/usr/bin/alacritty")
                    .remaining_args(args)
                    .command,
            );
        }
        "tbb" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .firejail_profile("thunderbird-beta")
                    .bin_path("/home/kalvin/Downloads/.thunderbird-beta/thunderbird")
                    .remaining_args(args)
                    .command,
            );
        }
        "vlk" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .firejail_profile("vlc")
                    .bin_path("/usr/bin/vlc")
                    .implicit_extra_args(&["--play-and-exit"])
                    .remaining_args(args)
                    .command,
            );
        }
        "e" => return Some(simple_firejail_command("evince", args)),
        _ => panic!("no handler for ``{}''", target),
    }
}

fn main() {
    if let Some(mut command) = init_command() {
        println!("{:#?}", command);
        command.exec();
    }
}
