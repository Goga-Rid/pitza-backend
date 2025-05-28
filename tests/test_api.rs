use actix_web::{test, App};
use pitza_backend::handlers;
use pitza_backend::db;
use actix_web::web;
use serde_json::json;

fn get_test_pool() -> pitza_backend::db::DbPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/pitza_test".to_string());
    db::init_pool(&database_url)
}

#[actix_rt::test]
async fn test_register_and_login() {
    let pool = get_test_pool();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::auth::register)
            .service(handlers::auth::login)
    ).await;

    // Регистрация нового пользователя
    let register_payload = json!({
        "email": "testuser@example.com",
        "password": "testpassword",
        "name": "Test User"
    });
    let req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&register_payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Логин
    let login_payload = json!({
        "email": "testuser@example.com",
        "password": "testpassword"
    });
    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&login_payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("token").is_some());
}

#[actix_rt::test]
async fn test_get_products() {
    let pool = get_test_pool();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(handlers::user::products::get_products)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/user/products")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
