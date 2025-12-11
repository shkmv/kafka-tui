// Database module - placeholder for Phase 2
use std::path::PathBuf;

use crate::error::AppResult;

pub struct Database {
    _db_path: PathBuf,
}

impl Database {
    pub async fn new(db_path: Option<PathBuf>) -> AppResult<Self> {
        let db_path = db_path.unwrap_or_else(|| {
            let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push("kafka-tui");
            std::fs::create_dir_all(&path).ok();
            path.push("kafka-tui.db");
            path
        });

        Ok(Self { _db_path: db_path })
    }
}
