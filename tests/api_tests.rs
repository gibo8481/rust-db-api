use actix_web::{test, web, App};
use fake::{Fake, Faker};
use rust_db_api::{
    db,
    handlers,
    models::{CreateItem, Item},
};
use serial_test::serial;
use sqlx::PgPool;
use uuid::Uuid;

async fn setup_test_app() -> (PgPool, test::TestApp) {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(handlers::health_check))
            .route("/items", web::post().to(handlers::create_item))
            .route("/items", web::get().to(handlers::list_items))
            .route("/items/{id}", web::get().to(handlers::get_item)),
    )
    .await;

    (pool, app)
}

#[actix_rt::test]
#[serial]
async fn test_health_check() {
    let (_pool, app) = setup_test_app().await;
    
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
}

#[actix_rt::test]
#[serial]
async fn test_create_and_get_item() {
    let (_pool, app) = setup_test_app().await;

    // Generate a random item name
    let item_name = format!("Test Item {}", Uuid::new_v4());
    
    // Create item
    let create_item = CreateItem {
        name: item_name.clone(),
        description: Some("Test description".to_string()),
    };
    
    let req = test::TestRequest::post()
        .uri("/items")
        .set_json(&create_item)
        .to_request();
    
    let resp: Item = test::call_and_read_body_json(&app, req).await;
    
    assert_eq!(resp.name, item_name);
    assert_eq!(resp.description, Some("Test description".to_string()));
    
    // Get the created item
    let req = test::TestRequest::get()
        .uri(&format!("/items/{}", resp.id))
        .to_request();
    
    let get_resp: Item = test::call_and_read_body_json(&app, req).await;
    
    assert_eq!(get_resp.id, resp.id);
    assert_eq!(get_resp.name, item_name);
}

#[actix_rt::test]
#[serial]
async fn test_list_items() {
    let (_pool, app) = setup_test_app().await;
    
    // Create a few items
    for _ in 0..3 {
        let create_item = CreateItem {
            name: Faker.fake::<String>(),
            description: Some(Faker.fake::<String>()),
        };
        
        let req = test::TestRequest::post()
            .uri("/items")
            .set_json(&create_item)
            .to_request();
        
        let _: Item = test::call_and_read_body_json(&app, req).await;
    }
    
    // Get list of items
    let req = test::TestRequest::get().uri("/items").to_request();
    let resp: Vec<Item> = test::call_and_read_body_json(&app, req).await;
    
    assert!(!resp.is_empty());
}

#[actix_rt::test]
#[serial]
async fn test_get_nonexistent_item() {
    let (_pool, app) = setup_test_app().await;
    
    let req = test::TestRequest::get()
        .uri("/items/999999")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 404);
}
