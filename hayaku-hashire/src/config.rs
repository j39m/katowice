use serde::Deserialize;
use std::option::Option;

#[derive(Deserialize)]
pub struct Config {
    pub executable: std::path::PathBuf,
    pub default_args: Option<Vec<String>>,

    pub systemd_run_params: Option<SystemdRunParams>,
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

fn executable_path(from_toml: &std::path::PathBuf) -> String {
    if from_toml.is_absolute() {
        return from_toml.to_str().unwrap().to_string();
    }
    let joined = home::home_dir().unwrap();
    joined.join(from_toml).to_str().unwrap().to_string()
}

impl CommandLine for Config {
    // Does not read args fed from invocation of this program. That
    // task is delegated to caller.
    fn as_args(&self) -> Option<Vec<String>> {
        let mut ret: Vec<String> = Vec::new();

        if let Some(systemd_run_params) = &self.systemd_run_params {
            if let Some(params) = systemd_run_params.as_args() {
                ret.extend(params);
            }
        }
        if let Some(bwrap_params) = &self.bwrap_params {
            if let Some(params) = bwrap_params.as_args() {
                ret.extend(params);
            }
        }
        ret.push(executable_path(&self.executable));
        if let Some(default_args) = &self.default_args {
            ret.extend(default_args.clone());
        }

        Some(ret)
    }
}

#[derive(Deserialize)]
pub struct SystemdRunParams {
    pub memory_high: u64,
    pub memory_max: u64,
    pub scope_name: Option<String>,
}

impl CommandLine for SystemdRunParams {
    fn as_args(&self) -> Option<Vec<String>> {
        // Both fields are required, so presence of this TOML block
        // means that we are definitely being cgroup'd, so there's no
        // case in which this returns `None`.
        let mut result = vec![
            String::from("/usr/bin/systemd-run"),
            String::from("--user"),
            String::from("--scope"),
            String::from("-p"),
            String::from("MemorySwapMax=0"),
            String::from("-p"),
            format!("MemoryHigh={}M", self.memory_high),
            String::from("-p"),
            format!("MemoryMax={}M", self.memory_max),
        ];
        if let Some(scope_name) = &self.scope_name {
            result.push(String::from("-u"));
            result.push(String::from(scope_name));
        }
        Some(result)
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

// For ease of readability only. Still vulnerable to me error.
type DefaultTrue = Option<bool>;

#[derive(Deserialize)]
pub struct BwrapParams {
    // Whether to implicitly RO-bind
    // * `/usr`,
    // * `/etc`,
    // * `/sys`, and
    //
    // `/etc` is especially important for `/etc/ld.so.conf`.
    pub use_default_ro_binds: DefaultTrue,

    // Whether to implicitly symlink
    // * `/bin` to `usr/bin` and
    // * `/lib64` to `usr/lib64`.
    pub use_default_symlinks: DefaultTrue,

    pub use_xdg_runtime_dir: DefaultTrue,
    pub create_tmpfs: DefaultTrue,
    pub new_session: DefaultTrue,

    // Whether to share the network namespace.
    // Defaults to false.
    pub share_net: Option<bool>,

    pub ro_binds: Option<BwrapBinds>,
    pub rw_binds: Option<BwrapBinds>,

    // These binds are all specified relative to `${HOME}`.
    pub home_ro_binds: Option<BwrapBinds>,
    pub home_rw_binds: Option<BwrapBinds>,

    pub dev_binds: Option<BwrapBinds>,

    pub setenv: Option<std::collections::HashMap<String, String>>,
    pub unsetenv: Option<Vec<String>>,
}

fn default_true_bool(opt: Option<bool>) -> bool {
    if let Some(false) = opt {
        return false;
    }
    true
}

impl CommandLine for BwrapParams {
    fn as_args(&self) -> Option<Vec<String>> {
        let mut ret: Vec<String> = vec![
            String::from("/usr/bin/bwrap"),
            String::from("--dev"),
            String::from("/dev"),
            String::from("--proc"),
            String::from("/proc"),
            String::from("--unshare-pid"),
        ];
        if default_true_bool(self.use_default_ro_binds) {
            ret.extend(arg_set_from("--ro-bind", None, "/usr", "/usr"));
            ret.extend(arg_set_from("--ro-bind", None, "/etc", "/etc"));
            ret.extend(arg_set_from("--ro-bind", None, "/sys", "/sys"));
        }
        if default_true_bool(self.use_default_symlinks) {
            ret.extend(arg_set_from("--symlink", None, "usr/bin", "/bin"));
            ret.extend(arg_set_from("--symlink", None, "usr/lib64", "/lib64"));
        }
        if default_true_bool(self.use_xdg_runtime_dir) {
            let xdg_dirs = xdg::BaseDirectories::new().unwrap();
            let xrd = xdg_dirs.get_runtime_directory().unwrap().to_str().unwrap();
            ret.extend(arg_set_from("--bind", None, xrd, xrd));
        }
        if default_true_bool(self.create_tmpfs) {
            ret.extend([String::from("--tmpfs"), String::from("/tmp")]);
        }
        if default_true_bool(self.new_session) {
            ret.push(String::from("--new-session"));
        }

        match &self.share_net {
            Some(true) => ret.push("--share-net".to_string()),
            _ => ret.push("--unshare-net".to_string()),
        };

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
            if let Some(args) = dev_binds.as_args_with_details("--dev-bind-try", None) {
                ret.extend(args);
            }
        }

        if let Some(setenv) = &self.setenv {
            for (var, val) in setenv.iter() {
                ret.extend([String::from("--setenv"), var.to_string(), val.to_string()]);
            }
        }
        if let Some(unsetenv) = &self.unsetenv {
            for var in unsetenv.iter() {
                ret.extend([String::from("--unsetenv"), var.to_string()]);
            }
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
