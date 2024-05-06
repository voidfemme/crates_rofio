fn read_from_cache(query: &str) -> Option<Vec<Crate>> {
    let cache_file_path = "/home/rsp/scripts/rust/rust_man/resources/data/cache.json"; // Use a single cache file
    if Path::new(&cache_file_path).exists() {
        if let Ok(content) = fs::read_to_string(cache_file_path) {
            if let Ok(cache) = serde_json::from_str::<Cache>(&content) {
                // Look for an entry matching the query with a valid timestamp
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                for entry in cache.entries {
                    if entry.query == query && now - entry.cached_time < 86400 {
                        // 24 hours validity
                        return Some(entry.crates);
                    }
                }
            }
        }
    }
    None
}

async fn fetch_and_cache_crates_data(query: &str) -> Result<()> {
    let fetched_crates = fetch_crates_data(query).await?;
    cache_results(query, &fetched_crates)?;
    Ok(())
}

async fn search_online_and_update_cache(query: &str) -> Result<()> {
    // Fetch crates data from the online source
    let packages = fetch_crates_data(query).await?;

    // Display the fetched crates with Rofi
    let selection = display_with_rofi(packages.clone())?;

    // Update the cache with the new results
    cache_results(query, &packages)?;

    // Handle the user's selection from the displayed results
    if let Some(selected_package) = packages.iter().find(|c| format!("{} - {}", c.name, c.max_version) == selection) {
        handle_crate_selection(&selected_package.name, &packages);
    }
    Ok(())
}
