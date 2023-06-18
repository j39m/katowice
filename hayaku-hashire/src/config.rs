use serde::Deserialize;
use std::option::Option;

#[derive(Deserialize)]
pub struct Config {
    pub executable: std::path::PathBuf,
    pub default_args: Option<toml::value::Array>,

    pub cgroup_params: Option<CgroupParams>,
    pub bwrap_params: Option<BwrapParams>,
}

#[derive(Deserialize)]
pub struct CgroupParams {
    pub memory_high: u64,
    pub memory_max: u64,
}

#[derive(Deserialize)]
pub struct BwrapParams {
    // Whether to implicitly RO-bind
    // * `/usr`,
    // * `/etc`,
    // * `/sys`, and
    // * `/run`.
    // Defaults to true.
    //
    // `/etc` is especially important for `/etc/ld.so.conf`.
    pub use_default_ro_binds: Option<bool>,

    // Whether to implicitly symlink
    // * `/bin` to `usr/bin` and
    // * `/lib64` to `usr/lib64`.
    // Defaults to true.
    pub use_default_symlinks: Option<bool>,

    // Whether to RW-bind `${XDG_RUNTIME_DIR}`.
    // Defaults to true.
    pub use_xdg_runtime_dir: Option<bool>,

    pub ro_binds: Option<BwrapBinds>,
    pub rw_binds: Option<BwrapBinds>,

    // These binds are all specified relative to `${HOME}`.
    pub home_ro_binds: Option<BwrapBinds>,
    pub home_rw_binds: Option<BwrapBinds>,
}

#[derive(Deserialize)]
pub struct BwrapBinds {
    // List of binds mapped into the mount namespace as themselves.
    pub list: Option<toml::value::Array>,

    // Map of binds whose destination mappings may be named differently
    // than their true names.
    pub map: Option<toml::value::Table>,
}
