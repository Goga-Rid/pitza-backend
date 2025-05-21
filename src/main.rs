use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use middleware::auth::AuthMiddleware;
use std::env;
use actix_cors::Cors;

mod db;
mod schema;
mod models;
mod handlers;
mod middleware;
mod utils;

type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Ensure JWT_SECRET is set
    env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);

    println!("🚀 Server running at http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_headers(["Authorization"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            // Public routes
            .service(handlers::auth::register)
            .service(handlers::auth::login)
            // Protected routes
            .wrap(AuthMiddleware)
            // Product routes
            .service(handlers::admin::products::get_products)
            .service(handlers::admin::products::create_product)
            .service(handlers::admin::products::delete_product)
            .service(handlers::admin::products::update_product) 
            // Order routes
            .service(handlers::admin::orders::get_orders)
            .service(handlers::admin::orders::get_order)
            .service(handlers::admin::orders::get_order_with_items)
            .service(handlers::admin::orders::update_order)
            .service(handlers::admin::orders::delete_order)
            .service(handlers::admin::orders::delete_order_item)    
            // User routes
            .service(handlers::user::products::get_products)
            .service(handlers::user::users::get_me)
            .service(handlers::user::users::update_user)
            .service(handlers::user::orders::get_orders)
            .service(handlers::user::orders::create_order)
            .service(handlers::user::reviews::create_review)
            .service(handlers::user::reviews::get_reviews_by_product)
            .service(handlers::user::orders::get_order_with_items_by_id)
            .service(handlers::user::favorites::create_favorite)
            .service(handlers::user::favorites::get_favorites)
            .service(handlers::user::favorites::delete_favorite)
            // validate token
            .service(handlers::auth::validate_token)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
