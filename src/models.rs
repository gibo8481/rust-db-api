use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Example model for a generic item
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// DTO for creating a new item
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: Option<String>,
}
