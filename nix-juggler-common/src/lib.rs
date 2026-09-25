use std::collections::{HashMap, HashSet};
use std::fs::{File, write};
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::Command;

use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize)]
pub struct Config {
    pub nix_module_path: String,
    pub pkg_source: String,
    pub writer_prefix: String,
}

#[derive(Deserialize)]
pub struct ManifestRoot {
    elements: HashMap<String, Package>,
}

#[derive(Deserialize)]
struct Package {
    #[serde(rename = "attrPath")]
    attr_path: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
}

pub fn get_config_path() -> String {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        format!("{xdg}/nix-juggler/config.toml")
    } else {
        println!("Config not found");
        "".to_string()
    }
}

// checks if str is formated like a valid pkg
pub fn is_valid_pkg_name(pkg: &str) -> bool {
    !pkg.is_empty()
        && pkg
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '+' | '-' | '.' | '_' | '?' | '='))
}

// checks if all packages passed as arguments are valid
pub fn is_valid_input(pkgs: &[String]) -> bool {
    for pkg in pkgs {
        if !is_valid_pkg_name(pkg) {
            return false;
        }
    }
    true
}

// loads the config
pub fn load_config(path: &str) -> Result<Config, ConfigError> {
    let contents = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}

// gets the dynamic name of a pkg. needed for nix profile remove
pub fn get_dynamic_name(pkg: &str) -> Option<String> {
    let cmd = Command::new("nix")
        .args(["profile", "list", "--json"])
        .output()
        .expect("failed to get nix profile list");

    let manifest_root: ManifestRoot = serde_json::from_slice(&cmd.stdout).ok()?;

    manifest_root
        .elements
        .into_iter()
        .find(|(_, package)| package.attr_path.ends_with(pkg))
        .map(|(name, _)| name)
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
