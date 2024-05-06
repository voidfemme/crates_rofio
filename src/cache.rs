// This file by voidfemme is released under CC0 1.0 Universal (CC0 1.0) Public Domain Dedication.
// https://creativecommons.org/publicdomain/zero/1.0
use crate::notification::{send_notification_error, send_notification_normal};
use crate::types::{Cache, CacheEntry, Crate};
use std::fs;
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const CACHE_FILE_PATH: &str = "/home/rsp/.config/crates_rofio/cache.json";

pub fn clear_cache() -> io::Result<()> {
    let path = Path::new(CACHE_FILE_PATH);
    if path.exists() {
        fs::remove_file(path)?;
        let _ =
            send_notification_normal("Cache Cleared", "The cache has been successfully cleared.");
    } else {
        let _ = send_notification_error("Cache Clear Error", "Cache file not found.");
    }
    Ok(())
}

pub fn cache_results(query: &str, crates: &Vec<Crate>) -> Result<(), io::Error> {
    let _ = send_notification_normal(
        "Caching results",
        &format!("Cached results saved to {}", CACHE_FILE_PATH),
    );
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut cache = match fs::read_to_string(CACHE_FILE_PATH) {
        Ok(content) => {
            serde_json::from_str::<Cache>(&content).unwrap_or_else(|_| Cache { entries: vec![] })
        }
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            send_notification_error(
                "Cache Initialization",
                "No cache file found, initializing a new cache...",
            )
            .expect("Failed to send notification");
            let new_cache = Cache { entries: vec![] };
            // Ensure directory exists
            if let Some(parent) = Path::new(CACHE_FILE_PATH).parent() {
                fs::create_dir_all(parent)?;
            }
            // Initialize cache file with new cache
            fs::write(CACHE_FILE_PATH, serde_json::to_string(&new_cache)?)?;
            new_cache
        }
        Err(e) => return Err(e),
    };

    // Update existing entry or append a new one
    if let Some(entry) = cache.entries.iter_mut().find(|e| e.query == query) {
        entry.crates = crates.clone();
        entry.cached_time = now;
    } else {
        cache.entries.push(CacheEntry {
            query: query.to_string(),
            crates: crates.clone(),
            cached_time: now,
        });
    }

    // Optionally, remove expired entries here

    let serialized = serde_json::to_string(&cache)?;
    fs::write(CACHE_FILE_PATH, serialized)?;
    Ok(())
}
pub fn find_crate_in_cache<'a>(
    selection: &'a str,
    cached_crates: &'a [Crate],
) -> Option<&'a Crate> {
    cached_crates
        .iter()
        .find(|&c| format!("{} - {}", c.name, c.max_version) == selection)
}

pub fn read_all_from_cache() -> Result<Vec<Crate>, io::Error> {
    let cache_file_path = Path::new(CACHE_FILE_PATH);

    if !cache_file_path.exists() {
        send_notification_error(
            "Cache File Error",
            "Cache file not found. Please check the cache configuration. (read_all_from_cache())",
        )
        .expect("Failed to send notification");

        // Create a new cache file at CACHE_FILE_PATH
        let new_cache = Cache { entries: vec![] };
        if let Some(parent) = Path::new(CACHE_FILE_PATH).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(CACHE_FILE_PATH, serde_json::to_string(&new_cache)?)?;
    }

    let content = fs::read_to_string(cache_file_path)?;
    let cache = serde_json::from_str::<Cache>(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    let all_crates = cache
        .entries
        .into_iter()
        .flat_map(|entry| entry.crates)
        .collect();

    Ok(all_crates)
}
