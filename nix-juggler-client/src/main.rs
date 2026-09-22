use nix_juggler_common::*;

use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(
    name = "nix-juggler",
    about = "Manage nix pkgs in your config and have them availible without having to rebuild.",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install one or more packages to the nix profile and the managed nix module.
    Install {
        /// Package names to install
        pkgs: Vec<String>,
    },
    /// Remove one or more packages from the nix profile and the managed nix module.
    Remove {
        /// Package names to remove
        pkgs: Vec<String>,
    },
    /// Remove all managed nix profile packages. Use after "nixos-rebuild switch to remove redundant
    /// nix profile packages."
    Clean,
}

// removes all installed nix profile pkgs
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
fn nix_profile_install(pkgs: Vec<String>, source: String) {
    for pkg in pkgs {
        let package: String = format!("{source}#{pkg}");
        Command::new("nix")
            .args(["profile", "add", &package])
            .status()
            .expect("failed to install to nix profile");
    }
}

// removes pkgs from nix profile
fn nix_profile_remove(pkgs: Vec<String>) {
    for pkg in pkgs {
        let dyn_name: String = match get_dynamic_name(&pkg) {
            Some(name) => name,
            None => {
                println!("No package found matching '{pkg}'");
                continue;
            }
        };

        Command::new("nix")
            .args(["profile", "remove", &dyn_name])
            .status()
            .expect("failed to remove from nix profile");
    }
}

// runs the writer as a child, forwarding it the exact args this binary was called with
fn spawn_writer(args: Vec<String>) -> std::io::Result<std::process::Child> {
    // Locate the writer binary next to this binary
    let exe = std::env::current_exe()?;
    let writer_path = exe.with_file_name("nix-juggler-writer");

    let child = Command::new(writer_path)
        .args(&args)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(child)
}

fn main() {
    let writer_args: Vec<String> = std::env::args().skip(1).collect();
    let config: Config = load_config("./config.toml").unwrap(); // TODO set reasonable config path

    let cli = Cli::parse();

    match cli.command {
        Commands::Install { pkgs } => nix_profile_install(pkgs, config.pkg_source),
        Commands::Remove { pkgs } => nix_profile_remove(pkgs),
        Commands::Clean => {
            nix_profile_clean(config.nix_module_path);
            std::process::exit(0); // no write needed
        }
    }

    // spawn writer, who updates the nix module
    match spawn_writer(writer_args) {
        Ok(mut child) => match child.wait() {
            Ok(status) if !status.success() => eprintln!("writer exited with: {status}"),
            Err(e) => eprintln!("failed to wait on writer: {e}"),
            _ => {}
        },
        Err(e) => eprintln!("failed to spawn writer: {e}"),
    }
}
