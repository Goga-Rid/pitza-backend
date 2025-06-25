use crate::db::DbPool;
use crate::models::Product;
use crate::schema::products::dsl::*;
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use std::collections::HashMap;

#[get("/api/products")]
pub async fn get_products(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    
    match products.load::<Product>(&mut conn) {
        Ok(products_list) => {
            let mut grouped: HashMap<String, Vec<Product>> = HashMap::new();
            for product in products_list {
                grouped
                    .entry(product.category.clone())
                    .or_default()
                    .push(product);
            }
            HttpResponse::Ok().json(grouped)
        },
        Err(e) => {
            log::error!("Failed to load products: {}", e);
            HttpResponse::InternalServerError().body("Failed to load products")
        }
    }
}