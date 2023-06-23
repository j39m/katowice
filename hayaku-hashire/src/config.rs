use serde::Deserialize;
use std::option::Option;

#[derive(Deserialize)]
pub struct Config {
    pub executable: std::path::PathBuf,
    pub default_args: Option<Vec<String>>,

    pub cgroup_params: Option<CgroupParams>,
    pub bwrap_params: Option<BwrapParams>,
}

pub trait CommandLine {
    // Expresses `self` as args for use on a command-line.
    fn as_args(&self) -> Option<Vec<String>> {
        return None;
    }

    // As above, but with extra args. Args are for genericism, perhaps
    // too much so -
    // *    `flag_name` assumes that a block is tied to one specific
    //      `--flag-name` and prefixes all (sets of) tokens with the
    //      same.
    // *    `join_prefix` provides a prefix that can be joined to
    //      every token. (Only really applicable to
    //      `BwrapParams.home_ro_binds` and `BwrapParams.home_rw_binds`.
    fn as_args_with_details(
        &self,
        _flag_name: &str,
        _join_prefix: Option<&str>,
    ) -> Option<Vec<String>> {
        return None;
    }
}

impl CommandLine for Config {
    // Does not read args fed from invocation of this program. That
    // task is delegated to caller.
    fn as_args(&self) -> Option<Vec<String>> {
        let mut ret: Vec<String> = Vec::new();

        if let Some(cgroup_params) = &self.cgroup_params {
            if let Some(params) = cgroup_params.as_args() {
                ret.extend(params);
            }
        }
        if let Some(bwrap_params) = &self.bwrap_params {
            if let Some(params) = bwrap_params.as_args() {
                ret.extend(params);
            }
        }
        ret.push(self.executable.to_str().unwrap().to_string());
        if let Some(default_args) = &self.default_args {
            ret.extend(default_args.clone());
        }

        Some(ret)
    }
}

#[derive(Deserialize)]
pub struct CgroupParams {
    pub memory_high: u64,
    pub memory_max: u64,
}

impl CommandLine for CgroupParams {
    fn as_args(&self) -> Option<Vec<String>> {
        // Both fields are required, so presence of this TOML block
        // means that we are definitely being cgroup'd, so there's no
        // case in which this returns `None`.
        Some(vec![
            String::from("/usr/bin/systemd-run"),
            String::from("--user"),
            String::from("--scope"),
            String::from("-p"),
            String::from("MemorySwapMax=0"),
            String::from("-p"),
            format!("MemoryHigh={}M", self.memory_high),
            String::from("-p"),
            format!("MemoryMax={}M", self.memory_max),
        ])
    }

    // Fall through to default for `as_args_for_details()`, as we don't
    // accept user-provided details for this struct.
}

// Helper function that returns a `Vec` like
// ```
// [ "--flag-name", "join_prefix/src", "join_prefix/dst" ]
// ```
fn arg_set_from(flag_name: &str, join_prefix: Option<&str>, src: &str, dst: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    ret.push(flag_name.to_string());

    let maybe_prefixed_src = match join_prefix {
        None => src.to_string(),
        Some(j) => std::path::PathBuf::from(j)
            .join(src)
            .to_str()
            .unwrap()
            .to_string(),
    };
    ret.push(maybe_prefixed_src);

    let maybe_prefixed_dst = match join_prefix {
        None => dst.to_string(),
        Some(j) => std::path::PathBuf::from(j)
            .join(dst)
            .to_str()
            .unwrap()
            .to_string(),
    };
    ret.push(maybe_prefixed_dst);

    return ret;
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

    pub dev_binds: Option<BwrapBinds>,
}

fn default_true_bool(opt: Option<bool>) -> bool {
    if let Some(false) = opt {
        return false;
    }
    true
}

impl CommandLine for BwrapParams {
    fn as_args(&self) -> Option<Vec<String>> {
        let mut ret: Vec<String> = Vec::new();

        ret.extend([String::from("--dev"), String::from("/dev")]);
        ret.extend([String::from("--proc"), String::from("/proc")]);
        if default_true_bool(self.use_default_ro_binds) {
            ret.extend(arg_set_from("--ro-bind", None, "/usr", "/usr"));
            ret.extend(arg_set_from("--ro-bind", None, "/etc", "/etc"));
            ret.extend(arg_set_from("--ro-bind", None, "/sys", "/sys"));
            ret.extend(arg_set_from("--ro-bind", None, "/run", "/run"));
        }
        if default_true_bool(self.use_default_symlinks) {
            ret.extend(arg_set_from("--symlink", None, "/bin", "usr/bin"));
            ret.extend(arg_set_from("--symlink", None, "/lib64", "usr/lib64"));
        }
        if default_true_bool(self.use_xdg_runtime_dir) {
            let xdg_dirs = xdg::BaseDirectories::new().unwrap();
            let xrd = xdg_dirs.get_runtime_directory().unwrap().to_str().unwrap();
            ret.extend(arg_set_from("--bind", None, xrd, xrd));
        }

        if let Some(ro_binds) = &self.ro_binds {
            if let Some(args) = ro_binds.as_args_with_details("--ro-bind", None) {
                ret.extend(args);
            }
        }
        if let Some(rw_binds) = &self.rw_binds {
            if let Some(args) = rw_binds.as_args_with_details("--bind", None) {
                ret.extend(args);
            }
        }
        let home_dir = home::home_dir().unwrap().to_str().unwrap().to_string();
        if let Some(home_ro_binds) = &self.home_ro_binds {
            if let Some(args) = home_ro_binds.as_args_with_details("--ro-bind", Some(&home_dir)) {
                ret.extend(args);
            }
        }
        if let Some(home_rw_binds) = &self.home_rw_binds {
            if let Some(args) = home_rw_binds.as_args_with_details("--bind", Some(&home_dir)) {
                ret.extend(args);
            }
        }
        if let Some(dev_binds) = &self.dev_binds {
            if let Some(args) = dev_binds.as_args_with_details("--dev-bind", None) {
                ret.extend(args);
            }
        }

        if ret.is_empty() {
            panic!("BUG: empty `BwrapParams`");
        }
        Some(ret)
    }
}

#[derive(Deserialize)]
pub struct BwrapBinds {
    // List of binds mapped into the mount namespace as themselves.
    pub list: Option<Vec<String>>,

    // Map of binds whose destination mappings may be named differently
    // than their true names.
    pub map: Option<std::collections::HashMap<String, String>>,
}

impl CommandLine for BwrapBinds {
    // `BwrapBinds` depends on the caller (`BwrapParams`) to define the
    // context in which it is being used. `as_args()` is not interesting
    // to implement, but `as_args_with_details()` is required.
    fn as_args_with_details(
        &self,
        flag_name: &str,
        join_prefix: Option<&str>,
    ) -> Option<Vec<String>> {
        if self.list.is_none() && self.map.is_none() {
            // This is...weird but not really crash-worthy?
            return None;
        }
        let mut args: Vec<String> = Vec::new();

        if let Some(list) = &self.list {
            for entry in list.iter() {
                args.extend(arg_set_from(flag_name, join_prefix, entry, entry));
            }
        }

        if let Some(map) = &self.map {
            for (key, val) in map.iter() {
                args.extend(arg_set_from(flag_name, join_prefix, key, val));
            }
        }

        Some(args)
    }
}
