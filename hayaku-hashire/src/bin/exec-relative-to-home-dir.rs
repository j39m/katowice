// A horrible workaround.
// Context:
// https://github.com/containers/bubblewrap/issues/693

use std::process::Command;

fn crank_pid_counter() {
    for _ in 0..rand::random_range(13..26) {
        let _ = Command::new("/bin/true").spawn();
    }
}

fn main() {
    crank_pid_counter();

    let mut args_less_the_first = std::env::args();
    // Pops the name of this executable.
    args_less_the_first.next();
    // Pops the name of the actual target.
    let target_relative_to_home_dir = args_less_the_first.next().unwrap();

    let joined = home::home_dir()
        .unwrap()
        .join(target_relative_to_home_dir)
        .to_str()
        .unwrap()
        .to_string();

    // Copies and collects the remaining argv.
    let args: Vec<String> = args_less_the_first.collect();

    let mut command = Command::new(joined);
    command.args(args);

    // `exec()` would defeat the whole purpose of `crank_pid_counter()`.
    let _ = command.spawn();
}
