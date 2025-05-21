use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::Product;
use crate::schema::products;
use crate::db::DbPool;
use std::collections::HashMap;

#[get("/api/user/products")]
pub async fn get_products(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let products: Vec<Product> = products::table.load(&mut conn).unwrap();    

    let mut grouped: HashMap<String, Vec<Product>> = HashMap::new();
    for product in products {
        grouped.entry(product.category.clone())
            .or_default()
            .push(product);
    }

    HttpResponse::Ok().json(grouped)
}