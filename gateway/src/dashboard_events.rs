use std::fs::OpenOptions;
use std::io::Write;

use tracing::warn;

use crate::models::dashboard::DashboardEvent;

pub fn append_dashboard_event(path: &str, event: &DashboardEvent) {
    let serialized = match serde_json::to_string(event) {
        Ok(value) => value,
        Err(error) => {
            warn!(error = %error, "Failed to serialize dashboard event");
            return;
        }
    };

    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut file) => {
            if let Err(error) = writeln!(file, "{}", serialized) {
                warn!(error = %error, path = %path, "Failed to append dashboard event");
            }
        }
        Err(error) => {
            warn!(error = %error, path = %path, "Failed to open dashboard event file");
        }
    }
}
