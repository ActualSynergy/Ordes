// Global configuration module

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub version: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub max_file_size: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_name: "Ordes".to_string(),
            version: "0.1.0".to_string(),
            database_url: "postgres://localhost/ordes".to_string(),
            jwt_secret: "your-secret-key".to_string(),
            max_file_size: 100 * 1024 * 1024, // 100 MB
        }
    }
}
