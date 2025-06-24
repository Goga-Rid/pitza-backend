use actix_web::{test, web, App};
use pitza_backend::{db, handlers};
use std::env;
use uuid::Uuid;

#[actix_web::test]
async fn test_register_and_login() {
    env::set_var("JWT_SECRET", "testsecret");
    env::set_var(
        "DATABASE_URL",
        "postgres://postgres:postgres@localhost/pitza_test",
    );
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = db::init_pool(&database_url);
    let unique_email = format!("integration_{}@example.com", Uuid::new_v4());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::auth::register)
            .service(handlers::auth::login),
    )
    .await;

    // Регистрация
    let req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(serde_json::json!({
            "email": unique_email,
            "password": "password123"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Логин
    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(serde_json::json!({
            "email": unique_email,
            "password": "password123"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_products() {
    env::set_var(
        "DATABASE_URL",
        "postgres://postgres:postgres@localhost/pitza_test",
    );
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = db::init_pool(&database_url);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::user::products::get_products),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/products")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
