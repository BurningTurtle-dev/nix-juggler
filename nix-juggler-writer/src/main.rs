use nix_juggler_common::*;

use std::collections::{HashSet, VecDeque};

// Formats the new output nix module
fn create_nix(pkgs: Vec<String>) {
    let head = "{ pkgs, ... }:\n{\nhome.pkgs = with pkgs; [\n";
    let tail = "];\n}";

    let mut buffer = String::new();
    buffer += head;

    for i in pkgs {
        buffer += &i;
        buffer += "\n";
    }

    buffer += tail;
    println!("{buffer}");
}

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    args.pop_front();

    let config: Config = load_config("./config.toml").unwrap(); // TODO set reasonable config path

    let operation = args.pop_front().unwrap();
    let new_pkgs: HashSet<String> = args.into_iter().collect();

    let existing_pkgs: HashSet<String> = match get_existing_pkgs(&config.nix_module_path) {
        Ok(existing_pkgs) => existing_pkgs,
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    };

    let pkgs = match operation.as_str() {
        "install" => existing_pkgs.union(&new_pkgs).cloned().collect(),
        "remove" => existing_pkgs.difference(&new_pkgs).cloned().collect(),
        _ => {
            eprintln!("{} is an invalid argument", operation);
            std::process::exit(1);
        }
    };

    create_nix(pkgs);
}
