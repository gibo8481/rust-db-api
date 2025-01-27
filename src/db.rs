use sqlx::PgPool;
use crate::models::{Item, CreateItem};

pub async fn create_item(pool: &PgPool, item: CreateItem) -> Result<Item, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"
        INSERT INTO items (name, description, created_at, updated_at)
        VALUES ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id, name, description, created_at, updated_at
        "#,
        item.name,
        item.description,
    )
    .fetch_one(pool)
    .await
}

pub async fn get_item(pool: &PgPool, id: i32) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM items
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn list_items(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM items
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}
