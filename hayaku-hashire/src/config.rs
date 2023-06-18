use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub executable: std::path::PathBuf,
    pub default_args: std::option::Option<toml::value::Array>,

    pub cgroup_params: std::option::Option<CgroupParams>,
    pub bwrap_params: std::option::Option<BwrapParams>,
}

#[derive(Deserialize)]
pub struct CgroupParams {
    pub memory_high: u64,
    pub memory_max: u64,
}

#[derive(Deserialize)]
pub struct BwrapParams {
    pub klaus: String,
}
