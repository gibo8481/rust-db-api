use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// Represents an item in the database
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Item {
    /// Unique identifier for the item
    pub id: i32,
    /// Name of the item
    pub name: String,
    /// Optional description of the item
    pub description: Option<String>,
    /// Timestamp when the item was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the item was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Data transfer object for creating a new item
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateItem {
    /// Name of the item (required)
    pub name: String,
    /// Optional description of the item
    pub description: Option<String>,
}
