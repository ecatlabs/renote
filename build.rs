use std::process::Command;

use chrono::prelude::*;

fn main() {
    let mut version = command("git", vec!["describe", "--tags", "--dirty"])
        .unwrap_or_else(|| format!("v{}", env!("CARGO_PKG_VERSION").to_string()));

    let short_version = version.clone();

    let commit =
        command("git", vec!["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "".to_string());

    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

    version = format!("{} Commit: {}-{}", version, commit, timestamp);

    println!("cargo:rustc-env=VERSION={}", short_version);
    println!("cargo:rustc-env=LONG_VERSION={}", version);
}

fn command(cmd: &str, args: impl IntoIterator<Item = &'static str>) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                return Some(output.stdout);
            }
            None
        })
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .map(|it| it.trim().to_string())
}
