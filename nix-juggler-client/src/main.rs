use nix_juggler_common::*;

use std::collections::VecDeque;
use std::env::args;
use std::process::{Command, Stdio};

// installs pkgs to nix profile
fn nix_profile_install(pkgs: VecDeque<String>, source: String) {
    for pkg in pkgs {
        let package: String = format!("{source}#{pkg}");
        Command::new("nix")
            .args(["profile", "add", &package])
            .status()
            .expect("failed to install to nix profile");
    }
}

// runs the writer as a child
fn spawn_writer(args: VecDeque<String>) -> std::io::Result<std::process::Child> {
    // Locate the writer binary next to this binary
    let exe = std::env::current_exe()?;
    let writer_path = exe.with_file_name("nix-juggler-writer");

    let child = Command::new(writer_path)
        .args(&args) // .args() takes IntoIterator<Item = AsRef<OsStr>>, Vec<String> works directly
        .stdin(Stdio::piped())
        //.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(child)
}

fn main() {
    let mut args: VecDeque<String> = args().collect();
    args.pop_front();
    let config: Config = load_config("./config.toml").unwrap(); // TODO set reasonable config path

    let mut new_pkgs = args.clone();
    new_pkgs.pop_front();

    nix_profile_install(new_pkgs, config.pkg_source);

    // spawn writer, who updates the nix module
    match spawn_writer(args) {
        Ok(mut child) => match child.wait() {
            Ok(status) if !status.success() => eprintln!("writer exited with: {status}"),
            Err(e) => eprintln!("failed to wait on writer: {e}"),
            _ => {}
        },
        Err(e) => eprintln!("failed to spawn writer: {e}"),
    }
}
