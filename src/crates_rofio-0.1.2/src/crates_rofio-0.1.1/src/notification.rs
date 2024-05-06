use std::io;
use std::process::Command;
use std::string::ToString;

pub fn _send_notification_success(crate_name: &str) -> Result<(), io::Error> {
    send_notification::<String>(
        "Opening documentation",
        Some(&format!(
            "Opening documentation or crates.io for selected crate: {}",
            crate_name
        )),
        Some("normal"),
        5000,
    )
}

pub fn send_notification_error(title: &str, message: &str) -> Result<(), std::io::Error> {
    send_notification::<String>(title, Some(message), Some("critical"), 5000)
}

pub fn send_notification_normal(title: &str, message: &str) -> Result<(), std::io::Error> {
    send_notification::<String>(title, Some(message), Some("normal"), 5000)
}

pub fn send_notification<T>(
    summary: &str,
    body: Option<&str>,
    urgency: Option<&str>,
    timeout: i32,
) -> Result<(), std::io::Error> {
    let mut command = Command::new("notify-send");
    command.args([
        "-u",
        urgency.unwrap_or("low"),
        "-t",
        &timeout.to_string(),
        summary,
    ]);
    if let Some(body) = body {
        command.arg(body);
    }

    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to send notification: {}", stderr),
        )
        .into());
    } else {
        Ok(())
    }
}
