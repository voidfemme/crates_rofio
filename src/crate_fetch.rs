use crate::cache::cache_results;
use crate::notification::send_notification_error;
use crate::rofi::display_with_rofi;
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

pub async fn fetch_and_display_results(query: &str) -> Result<()> {
    match fetch_crates_data(query).await {
        Ok(crates) => {
            println!("Updating cache with new results.");
            // Update cache with new results.
            if let Err(e) = cache_results(query, &crates) {
                send_notification_error("Cache Error", "Could not update cache with results")?;
                eprintln!("Failed to cache results: {}", e);
            }

            // Display newly fetched crates with an option to search online again.
            display_with_rofi(crates)?;
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
