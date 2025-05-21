use actix_web::{delete, get, post, put, web, HttpResponse, Result, HttpRequest};
use diesel::prelude::*;
use crate::models::{Product, NewProduct};
use crate::schema::products;
use crate::db::DbPool;
use std::collections::HashMap;
use crate::utils::is_admin;

#[get("/api/admin/products")]
pub async fn get_products(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse> {
    if !is_admin(&req) {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let mut conn = pool.get().unwrap();
    let products: Vec<Product> = products::table.load(&mut conn).unwrap();    

    let mut grouped: HashMap<String, Vec<Product>> = HashMap::new();
    for product in products {
        grouped.entry(product.category.clone())
            .or_default()
            .push(product);
    }

    Ok(HttpResponse::Ok().json(grouped))
}

#[post("/api/admin/products")]
pub async fn create_product(
    pool: web::Data<DbPool>,
    product: web::Json<NewProduct>,
    req: HttpRequest
) -> Result<HttpResponse> {
    if !is_admin(&req) {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let mut conn = pool.get().unwrap();

    let product = diesel::insert_into(products::table)
        .values(&*product)
        .get_result::<Product>(&mut conn)
        .unwrap();

    Ok(HttpResponse::Created().json(product))
}

#[put("/api/admin/products/{id}")]
pub async fn update_product(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    data: web::Json<NewProduct>,
    req: HttpRequest
) -> Result<HttpResponse> {
    if !is_admin(&req) {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let mut conn = pool.get().unwrap();
    let id = path.into_inner();

    diesel::update(products::table.find(id))
        .set(&*data)
        .execute(&mut conn)
        .unwrap();

    Ok(HttpResponse::Ok().body("Updated"))
}

#[delete("/api/admin/products/{id}")]
pub async fn delete_product(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    req: HttpRequest
) -> Result<HttpResponse> {
    if !is_admin(&req) {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let mut conn = pool.get().unwrap();
    let id = path.into_inner();

    diesel::delete(products::table.find(id))
        .execute(&mut conn)
        .unwrap();

    Ok(HttpResponse::Ok().body("Deleted"))
}
