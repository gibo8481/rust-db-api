use actix_web::{web, HttpResponse, Responder};
use crate::{models::CreateItem, db};
use sqlx::PgPool;

pub async fn create_item(
    pool: web::Data<PgPool>,
    item: web::Json<CreateItem>,
) -> impl Responder {
    match db::create_item(&pool, item.into_inner()).await {
        Ok(item) => HttpResponse::Created().json(item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

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

pub async fn list_items(
    pool: web::Data<PgPool>,
) -> impl Responder {
    match db::list_items(&pool).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
