mod config;
use crate::config::Config;

fn main() {
    println!("Hello, world!");
    let config: Config = toml::from_str(r#"
        executable = "klaus/tama"
        default_args = [
            "Hello",
            "there!",
        ]

        [cgroup_params]
        memory_high = 65
    "#).unwrap();
    assert_eq!(config.cgroup_params.unwrap().memory_max, 0);
}
