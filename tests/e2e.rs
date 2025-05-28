use actix_web::{http::header, test, web, App};
use pitza_backend::{db, handlers, middleware};
use std::env;
use uuid::Uuid;

#[actix_web::test]
async fn test_register_login_and_get_profile() {
    // Устанавливаем переменные окружения для теста
    env::set_var("JWT_SECRET", "testsecret");
    env::set_var(
        "DATABASE_URL",
        "postgres://postgres:postgres@localhost/pitza_test",
    );
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = db::init_pool(&database_url);

    // Генерируем уникальный email
    let unique_email = format!("e2e_{}@example.com", Uuid::new_v4());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::auth::register)
            .service(handlers::auth::login)
            .wrap(middleware::auth::AuthMiddleware)
            .service(handlers::user::users::get_me),
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
    let body: serde_json::Value = test::read_body_json(resp).await;
    let token = body["token"].as_str().unwrap();

    // Получение профиля пользователя (защищённый эндпоинт)
    let req = test::TestRequest::get()
        .uri("/api/me")
        .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
