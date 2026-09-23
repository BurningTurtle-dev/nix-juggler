use nix_juggler_common::*;

use std::collections::{HashSet, VecDeque};

// Formats the new output nix module
fn create_nix(mut pkgs: Vec<String>) -> String {
    let head = "{ pkgs, ... }:\n{\nhome.packages = with pkgs; [\n";
    let tail = "];\n}";

    let mut buffer = String::new();
    buffer += head;

    pkgs.sort();
    for i in pkgs {
        buffer += &i;
        buffer += "\n";
    }

    buffer += tail;
    buffer
}

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    args.pop_front();

    let config: Config = load_config("./config.toml").unwrap(); // TODO set reasonable config path

    let operation = args.pop_front().unwrap();
    let new_pkgs: HashSet<String> = args.into_iter().collect();

    let input_pkgs: Vec<String> = new_pkgs.clone().into_iter().collect();
    if !is_valid_input(&input_pkgs) {
        std::process::exit(1);
    }

    let existing_pkgs: HashSet<String> = match get_existing_pkgs(&config.nix_module_path) {
        Ok(existing_pkgs) => existing_pkgs,
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    };

    let pkgs: Vec<String> = match operation.as_str() {
        "install" => existing_pkgs.union(&new_pkgs).cloned().collect(),
        "remove" => existing_pkgs.difference(&new_pkgs).cloned().collect(),
        _ => {
            eprintln!("{} is an invalid argument", operation);
            std::process::exit(1);
        }
    };

    let updated_module: String = create_nix(pkgs);
    if !write_nix_module(&config.nix_module_path, updated_module) {
        std::process::exit(1);
    }
    println!("written updated module");
}
