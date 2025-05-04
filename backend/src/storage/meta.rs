use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub id: String,
    pub title: String,
    pub created_at: String,
}

impl Meta {
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}