mod cache;
mod config;
mod crate_fetch;
mod notification;
mod rofi;
mod types;

use crate::cache::{find_crate_in_cache, read_all_from_cache};
use crate::crate_fetch::fetch_and_display_results;
use crate::notification::{send_notification, send_notification_error};
use crate::rofi::{display_with_rofi, handle_crate_selection, prompt_and_fetch};
use crate::types::Crate;

use std::error::Error;
use tokio::runtime::Runtime;
use tracing::{debug, warn};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

fn main() {
    // Initialize the logging subscriber with environment-based filtering and span events
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // Creates a new Tokio runtime for running async tasks
    let rt = Runtime::new().unwrap();

    // Read cache data
    match read_all_from_cache() {
        Ok(cached_crates) => handle_user_selection(&rt, cached_crates),
        Err(e) => {
            eprintln!("Failed to read cache: {}", e);
        }
    }
}

fn handle_user_selection(rt: &Runtime, current_crates: Vec<Crate>) {
    debug!("Displaying {} crates to the user", current_crates.len());

    match display_with_rofi(current_crates.clone()) {
        Ok(selection) if selection == "Search Online" => {
            // When the user selects "Search Online", fetch new crates and display them
            match prompt_and_fetch(rt) {
                Ok(fetched_crates) if !fetched_crates.is_empty() => {
                    handle_user_selection(rt, fetched_crates);
                }
                Ok(_) => {
                    handle_user_selection(rt, current_crates);
                }
                Err(e) => {
                    report_error("Error fetching crates", &e.to_string());
                    return;
                }
            }
        }
        Ok(selection) => process_selection(rt, &selection, &current_crates),
        Err(e) => {
            report_error("Error displaying crates with Rofi", &e.to_string());
            return;
        }
    }
}

// Processes the user's selection, either fetching more information or handling the selection
// directly.
fn process_selection(rt: &Runtime, selection: &str, cached_crates: &[Crate]) {
    debug!("process_selection(): Processing selection: {}", selection);

    // Check if the selected crate is in the cache and handle it, otherwise, fetch the crate from
    // online
    if let Some(selected_crate) = find_crate_in_cache(selection, cached_crates) {
        handle_crate_selection(&selected_crate.name, cached_crates);
    } else {
        match rt.block_on(fetch_and_process_selection(selection)) {
            Ok(fetched_crates) if !fetched_crates.is_empty() => {
                handle_user_selection(rt, fetched_crates);
            }
            Ok(_) => debug!("No crates fetched for selection: {}", selection),
            Err(e) => report_error("Error fetching crates for new query", &e.to_string()),
        }
    }
}

// Fetches crate information based on the user's selection and displays results
async fn fetch_and_process_selection(selection: &str) -> Result<Vec<Crate>, Box<dyn Error>> {
    debug!("Fetching and processing selection: {}", selection);

    // Sends a notification to the user indicating that fetching has started
    let _ = send_notification::<String>(
        "Fetching results!",
        Some(&format!("For: {}", selection)),
        Some("normal"),
        10000,
    );

    // Asynchronously fetches and displays results for the selected crate.
    fetch_and_display_results(selection).await
}

// Reports errors both to the console and via a notification system
fn report_error(context: &str, message: &str) {
    warn!("Reporting error: {} - {}", context, message);
    eprintln!("{}: {}", context, message);
    let _ = send_notification_error(context, message);
}
