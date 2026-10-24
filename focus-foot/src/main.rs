use serde::Deserialize;
use std::os::unix::process::CommandExt;

#[derive(Deserialize)]
struct Window {
    id: u64,
    app_id: String,
    #[serde(flatten)]
    _other: serde_json::Map<String, serde_json::Value>,
}

fn main() {
    let windows_utf8 = String::from_utf8(
        std::process::Command::new("niri")
            .args(["msg", "--json", "windows"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let parsed_windows: Vec<Window> = serde_json::from_str(&windows_utf8).unwrap();
    let foot_id: Vec<u64> = parsed_windows
        .into_iter()
        .filter_map(|w| (w.app_id == "foot").then(|| w.id))
        .collect();
    assert_eq!(foot_id.len(), 1usize);

    // `exec()` does not usually return, so `eprintln!()` usually never
    // executes.
    eprintln!(
        "{}",
        std::process::Command::new("niri")
            .args([
                "msg",
                "action",
                "focus-window",
                "--id",
                &foot_id[0].to_string()
            ])
            .exec()
    );
}
