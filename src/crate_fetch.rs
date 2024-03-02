use crate::cache::cache_results;
use crate::notification::send_notification_error;
use crate::types::{Crate, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    crates: Vec<Crate>,
}

pub async fn fetch_crates_data(query: &str) -> Result<Vec<Crate>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://crates.io/api/v1/crates?page=1&per_page=10&q={}",
        query
    );

    let res = client
        .get(&url)
        .header("User-Agent", "RustCrateSearchTool/0.1")
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    Ok(res.crates)
}

pub async fn fetch_and_display_results(query: &str) -> Result<Vec<Crate>> {
    let fetched_crates = fetch_crates_data(query).await?;

    println!("Updating cache with new results.");
    // Update cache with new results.
    if let Err(e) = cache_results(query, &fetched_crates) {
        send_notification_error("Cache Error", "Could not update cache with results")?;
        eprintln!("Failed to cache results: {}", e);
    }

    Ok(fetched_crates)
}
