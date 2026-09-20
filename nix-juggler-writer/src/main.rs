use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

// Reads the existing pkgs from the nix module
fn get_existing_pkgs() -> Result<HashSet<String>, io::Error> {
    let mut pkgs: HashSet<String> = HashSet::new();
    let mut body_reached: bool = false;

    let file = File::open("test.nix")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();
        if trimmed_line == "];" {
            body_reached = false;
        } else if body_reached {
            pkgs.insert(trimmed_line.to_string());
        } else if trimmed_line == "home.packages = with pkgs; [" {
            body_reached = true;
        }
    }

    Ok(pkgs)
}

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

    let operation = args.pop_front().unwrap();
    let new_pkgs: HashSet<String> = args.into_iter().collect();

    let existing_pkgs: HashSet<String> = match get_existing_pkgs() {
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
