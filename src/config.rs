// This file by voidfemme is released under CC0 1.0 Universal (CC0 1.0) Public Domain Dedication.
// https://creativecommons.org/publicdomain/zero/1.0
use crate::cache::CACHE_FILE_PATH;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

#[allow(dead_code)]
pub const CONFIG_FILE_PATH: &str = "/home/rsp/.config/crates_rofio/config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub cache_file: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_file: PathBuf::from(CACHE_FILE_PATH),
        }
    }
}

#[allow(dead_code)]
fn load_config() -> Config {
    let config_path = get_config_path().unwrap_or_else(|| PathBuf::from(CONFIG_FILE_PATH));
    if config_path.exists() {
        let config_str = fs::read_to_string(config_path).expect("Failed to read config file");
        toml::from_str(&config_str).expect("Failed to parse config file")
    } else {
        Config::default()
    }
}

#[allow(dead_code)]
fn get_config_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "voidfemme", "crates_rofio") {
        let config_dir = proj_dirs.config_dir();
        let config_file_path = config_dir.join("config.toml");
        if config_file_path.exists() {
            return Some(config_file_path);
        }
    }
    None
}
