use nix_juggler_common::*;

use std::collections::{HashSet, VecDeque};
use std::env::args;
use std::process::{Command, Stdio};

fn nix_profile_clean(path: String) {
    let existing_pkgs: HashSet<String> = match get_existing_pkgs(&path) {
        Ok(existing_pkgs) => existing_pkgs,
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    };

    nix_profile_remove(existing_pkgs.into_iter().collect());
}

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

// removes pkgs to nix profile
fn nix_profile_remove(pkgs: VecDeque<String>) {
    for pkg in pkgs {
        Command::new("nix")
            .args(["profile", "remove", &pkg])
            .status()
            .expect("failed to remove to nix profile");
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
    let operation = new_pkgs.pop_front().unwrap();

    // nix profile operations
    match operation.as_str() {
        "install" => nix_profile_install(new_pkgs, config.pkg_source),
        "remove" => nix_profile_remove(new_pkgs),
        "clean" => nix_profile_clean(config.nix_module_path),
        _ => {
            eprintln!("{} is an invalid argument", operation);
            std::process::exit(1);
        }
    };

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
