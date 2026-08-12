// Git-like versioning system

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    pub project_id: String,
    pub parent_version_id: Option<String>,
    pub author: String,
    pub message: String,
    pub content: serde_json::Value,
    pub created_at: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub head_version_id: String,
    pub created_at: String,
}

impl Version {
    pub fn new(
        project_id: String,
        author: String,
        message: String,
        content: serde_json::Value,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id,
            parent_version_id: None,
            author,
            message,
            content,
            created_at: chrono::Utc::now().to_rfc3339(),
            tags: Vec::new(),
        }
    }
}
