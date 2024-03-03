use serde::{Deserialize, Serialize};
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedData {
    pub crates: Vec<Crate>,
    pub cached_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheEntry {
    pub query: String,
    pub crates: Vec<Crate>,
    pub cached_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    pub entries: Vec<CacheEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crate {
    pub name: String,
    pub max_version: String,
    pub description: Option<String>,
    pub documentation: Option<String>,
}
