use crate::crate_fetch::fetch_and_display_results;
use crate::types::{Crate, Result};

use std::io::{self, Write};
use std::process::{Command, Stdio};
use tokio::runtime::Runtime;

pub fn get_user_input() -> Result<Option<String>> {
    let output = Command::new("rofi")
        .arg("-dmenu")
        .arg("-p")
        .arg("Enter Query: ")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    if output.status.success() {
        let input = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if input.is_empty() {
            Ok(None)
        } else {
            Ok(Some(input))
        }
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Failed to get input from rofi",
        )))
    }
}

pub fn display_with_rofi(results: Vec<Crate>) -> io::Result<String> {
    let mut formatted_results = "Search Online".to_string();
    for c in results.iter() {
        let entry = format!("\n{} - {}", c.name, c.max_version);
        formatted_results.push_str(&entry);
    }

    let mut rofi = Command::new("rofi")
        .arg("-dmenu")
        .arg("-p")
        .arg("Crate Search:")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(ref mut stdin) = rofi.stdin {
        stdin.write_all(formatted_results.as_bytes())?;
    }

    let output = rofi.wait_with_output()?;
    if output.status.success() {
        let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(selection)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to get selection",
        ))
    }
}

// Assuming `selection` is a crate name obtained from the user input
pub fn handle_crate_selection(crate_name: &str, cached_crates: &[Crate]) {
    if let Some(selected_crate) = cached_crates.iter().find(|&c| c.name == crate_name) {
        match &selected_crate.documentation {
            Some(doc_url) if !doc_url.is_empty() => {
                println!("Opening documentation: {}", doc_url);
                open_url(doc_url);
            }
            _ => {
                let crates_io_url = format!("https://crates.io/crates/{}", crate_name);
                println!("Opening crates.io page: {}", crates_io_url);
                open_url(&crates_io_url);
            }
        }
    } else {
        println!("Crate not found in cache. Fetching from crates.io...");
        // Fetch crate data from crates.io here if necessary
    }
}

pub fn prompt_and_fetch(rt: &Runtime) -> Result<Vec<Crate>> {
    match get_user_input() {
        Ok(Some(new_query)) if !new_query.is_empty() => {
            println!("Fetching results for: {}", new_query);
            match rt.block_on(fetch_and_display_results(&new_query)) {
                Ok(crates) => Ok(crates),
                Err(e) => Err(e),
            }
        }
        Ok(Some(_)) | Ok(None) => {
            println!("No valid query provided. Exiting.");
            Ok(vec![])
        }
        Err(e) => {
            eprintln!("Failed to get new query: {}", e);
            Err(e)
        }
    }
}

// Function to open a URL in the default web browser
pub fn open_url(url: &str) {
    if let Err(e) = webbrowser::open(url) {
        eprintln!("Failed to open URL: {}", e);
    }
}

#[allow(dead_code)]
pub async fn fetch_and_display(new_query: String) {
    println!("Fetching results for: {}", new_query);
    match fetch_and_display_results(&new_query).await {
        Ok(_) => (),
        Err(e) => eprintln!("Error fetching or displaying results: {}", e),
    }
}
