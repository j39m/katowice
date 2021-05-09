use std::io::Write;
use std::option::Option;
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

const MPV: &'static str = "/usr/bin/mpv";
const MPV_PROFILE: &'static str = "/etc/firejail/mpv.profile";

const TERM: &'static str = "/usr/bin/alacritty";
const TERM_PROFILE: &'static str = "/etc/firejail/x-terminal-emulator.profile";

const THUNDERBIRD: &'static str = "/home/kalvin/Downloads/.thunderbird-beta/thunderbird";
const THUNDERBIRD_PROFILE: &'static str = "/etc/firejail/thunderbird-beta.profile";

const QUODLIBET_FIFO: &'static str = "/home/kalvin/.config/quodlibet/control";

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
            command: std::process::Command::new(SYSTEMD_RUN),
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
        self.command.args(&[FIREJAIL, "--ignore=seccomp"]);
        self.command.arg(format!("--profile={}", path));
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
    let mut command = Command::new(FIREJAIL);

    command.arg(format!("--profile=/etc/firejail/{}.profile", target));
    command.arg(format!("/usr/bin/{}", target));
    command.args(args);

    command
}

fn quodlibet_command(args: Vec<String>) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(QUODLIBET_FIFO)
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
        "ff" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .memory_high(FIREFOX_MEMORY_HIGH)
                    .memory_max(FIREFOX_MEMORY_MAX)
                    .firejail_profile(FIREFOX_PROFILE)
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
                    .firejail_profile(KEIRA_PROFILE)
                    .bin_path(KEIRA)
                    .implicit_extra_args(&["-P", "nightly"])
                    .remaining_args(args)
                    .command,
            );
        }
        "npv" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .firejail_profile(MPV_PROFILE)
                    .bin_path(MPV)
                    .implicit_extra_args(&["--pulse-buffer=13"])
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
                    .firejail_profile(FIREFOX_PROFILE)
                    .bin_path(FIREFOX)
                    .implicit_extra_args(&["-P", FIREFOX_MOZILLA_SFW_PROFILE])
                    .remaining_args(formatted_args)
                    .command,
            );
        }
        "t" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .firejail_profile(TERM_PROFILE)
                    .bin_path(TERM)
                    .remaining_args(args)
                    .command,
            );
        }
        "tbb" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .firejail_profile(THUNDERBIRD_PROFILE)
                    .bin_path(THUNDERBIRD)
                    .remaining_args(args)
                    .command,
            );
        }
        "vlk" => {
            return Some(
                CgroupedFirejailedCommand::new()
                    .firejail_profile("/etc/firejail/vlc.profile")
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
