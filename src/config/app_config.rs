use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// Default connection profile name
    pub default_connection: Option<String>,

    /// Theme (currently only "catppuccin-mocha")
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Show internal topics by default
    #[serde(default)]
    pub show_internal_topics: bool,

    /// Maximum messages to display
    #[serde(default = "default_max_messages")]
    pub max_messages: usize,

    /// Log level
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_theme() -> String {
    "catppuccin-mocha".to_string()
}

fn default_max_messages() -> usize {
    1000
}

fn default_log_level() -> String {
    "warn".to_string()
}

impl AppConfig {
    pub fn load(path: Option<PathBuf>) -> anyhow::Result<Self> {
        let config_path = path.unwrap_or_else(|| {
            let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
            p.push("kafka-tui");
            p.push("config.toml");
            p
        });

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: AppConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(AppConfig::default())
        }
    }

    pub fn save(&self, path: Option<PathBuf>) -> anyhow::Result<()> {
        let config_path = path.unwrap_or_else(|| {
            let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
            p.push("kafka-tui");
            std::fs::create_dir_all(&p).ok();
            p.push("config.toml");
            p
        });

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }
}
