use serde::Deserialize;

#[derive(Deserialize)]
struct Window {
    id: u64,
    app_id: String,
    #[serde(flatten)]
    _other: serde_json::Map<String, serde_json::Value>,
}

fn get_first_id_for(app_id: &str) -> u64 {
    let windows_utf8 = String::from_utf8(
        std::process::Command::new("niri")
            .args(["msg", "--json", "windows"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let parsed_windows: Vec<Window> = serde_json::from_str(&windows_utf8).unwrap();
    let filtered: std::collections::BTreeSet<u64> = parsed_windows
        .into_iter()
        .filter_map(|w| (w.app_id == app_id).then(|| w.id))
        .collect();
    assert!(!filtered.is_empty());
    *filtered.first().unwrap()
}

fn main() {
    let app_id = get_first_id_for("foot");

    use std::os::unix::process::CommandExt;
    // `exec()` does not usually return, so `eprintln!()` usually never
    // executes.
    eprintln!(
        "{}",
        std::process::Command::new("niri")
            .args(["msg", "action", "focus-window", "--id", &app_id.to_string()])
            .exec()
    );
}
