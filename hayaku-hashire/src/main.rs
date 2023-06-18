mod config;
use crate::config::Config;

fn main() {
    println!("Hello, world!");
    let config: Config = toml::from_str(
        r#"
        executable = "klaus/tama"
        default_args = [
            "Hello",
            "there!",
        ]

        [cgroup_params]
        memory_high = 65
        memory_max = 0

        [bwrap_params.ro_binds]
        list = [
            "Good",
            "god!",
        ]

        [bwrap_params.ro_binds.map]
        hello = "there"
    "#,
    )
    .unwrap();
    assert_eq!(config.cgroup_params.unwrap().memory_max, 0);
    for item in config
        .bwrap_params
        .as_ref()
        .unwrap()
        .ro_binds
        .as_ref()
        .unwrap()
        .list
        .as_ref()
        .unwrap()
        .iter()
    {
        println!("{}", item);
    }
    for (key, val) in config
        .bwrap_params
        .unwrap()
        .ro_binds
        .unwrap()
        .map
        .unwrap()
        .iter()
    {
        println!("{}: {}", key, val);
    }
}
