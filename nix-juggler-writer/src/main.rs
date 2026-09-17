use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn get_existing_pkgs() -> Result<Vec<String>, io::Error> {
    let mut pkgs: Vec<String> = Vec::new();
    let mut body_reached: bool = false;

    let file = File::open("test.nix")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();
        if trimmed_line == "];" {
            body_reached = false;
        } else if body_reached {
            pkgs.push(trimmed_line.to_string());
        } else if trimmed_line == "home.packages = with pkgs; [" {
            body_reached = true;
        }
    }

    Ok(pkgs)
}

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
    let pkgs: Vec<String> = match get_existing_pkgs() {
        Ok(pkgs) => pkgs,
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    };

    create_nix(pkgs);
}
