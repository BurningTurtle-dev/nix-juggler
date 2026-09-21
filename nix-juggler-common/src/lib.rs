use std::collections::HashSet;
use std::fs::{File, write};
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize)]
pub struct Config {
    pub nix_module_path: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
}

pub fn load_config(path: &str) -> Result<Config, ConfigError> {
    let contents = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}

// Reads the existing pkgs from the nix module
pub fn get_existing_pkgs(path: &str) -> Result<HashSet<String>, io::Error> {
    let mut pkgs: HashSet<String> = HashSet::new();
    let mut body_reached: bool = false;

    let file = File::open(path)?;
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

// Writes formated nix module to file
pub fn write_nix_module(path: &str, contents: String) -> bool {
    match write(path, contents) {
        Ok(()) => true,
        Err(e) => {
            println!("Failed to write nix module: {e}");
            false
        }
    }
}
