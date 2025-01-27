mod models;
mod db;
mod handlers;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

// Basic health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    // Set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Get host and port from environment variables or use defaults
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    println!("Starting server at http://{}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
            .route("/items", web::post().to(handlers::create_item))
            .route("/items", web::get().to(handlers::list_items))
            .route("/items/{id}", web::get().to(handlers::get_item))
    })
    .bind((host, port))?
    .run()
    .await
}
