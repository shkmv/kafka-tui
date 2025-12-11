use std::fs;
use std::path::PathBuf;

use crate::app::state::ConnectionProfile;
use crate::error::{AppError, AppResult};

/// Get the path to the connections file
fn get_connections_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kafka-tui");

    // Ensure directory exists
    let _ = fs::create_dir_all(&config_dir);

    config_dir.join("connections.json")
}

/// Load all saved connection profiles
pub fn load_connections() -> AppResult<Vec<ConnectionProfile>> {
    let path = get_connections_path();

    if !path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| AppError::Config(format!("Failed to read connections file: {}", e)))?;

    if content.trim().is_empty() {
        return Ok(vec![]);
    }

    let profiles: Vec<ConnectionProfile> = serde_json::from_str(&content)
        .map_err(|e| AppError::Config(format!("Failed to parse connections: {}", e)))?;

    Ok(profiles)
}

/// Save a connection profile (add or update)
pub fn save_connection(profile: &ConnectionProfile) -> AppResult<()> {
    let mut profiles = load_connections().unwrap_or_default();

    // Check if profile with this ID already exists
    if let Some(existing) = profiles.iter_mut().find(|p| p.id == profile.id) {
        *existing = profile.clone();
    } else {
        profiles.push(profile.clone());
    }

    save_all_connections(&profiles)
}

/// Delete a connection profile by ID
pub fn delete_connection(id: uuid::Uuid) -> AppResult<()> {
    let mut profiles = load_connections().unwrap_or_default();
    profiles.retain(|p| p.id != id);
    save_all_connections(&profiles)
}

/// Save all connections to file
fn save_all_connections(profiles: &[ConnectionProfile]) -> AppResult<()> {
    let path = get_connections_path();

    let content = serde_json::to_string_pretty(profiles)
        .map_err(|e| AppError::Config(format!("Failed to serialize connections: {}", e)))?;

    fs::write(&path, content)
        .map_err(|e| AppError::Config(format!("Failed to write connections file: {}", e)))?;

    Ok(())
}
