use actix_web::{web, HttpResponse, Responder};
use crate::{models::{CreateItem, Item}, db};
use sqlx::PgPool;

/// Create a new item
#[utoipa::path(
    post,
    path = "/items",
    request_body = CreateItem,
    responses(
        (status = 201, description = "Item created successfully", body = Item),
        (status = 500, description = "Internal server error")
    ),
    tags("Items")
)]
pub async fn create_item(
    pool: web::Data<PgPool>,
    item: web::Json<CreateItem>,
) -> impl Responder {
    match db::create_item(&pool, item.into_inner()).await {
        Ok(item) => HttpResponse::Created().json(item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Get an item by ID
#[utoipa::path(
    get,
    path = "/items/{id}",
    params(
        ("id" = i32, Path, description = "Item ID")
    ),
    responses(
        (status = 200, description = "Item found", body = Item),
        (status = 404, description = "Item not found"),
        (status = 500, description = "Internal server error")
    ),
    tags("Items")
)]
pub async fn get_item(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {
    match db::get_item(&pool, id.into_inner()).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// List all items
#[utoipa::path(
    get,
    path = "/items",
    responses(
        (status = 200, description = "List of items", body = Vec<Item>),
        (status = 500, description = "Internal server error")
    ),
    tags("Items")
)]
pub async fn list_items(
    pool: web::Data<PgPool>,
) -> impl Responder {
    match db::list_items(&pool).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
