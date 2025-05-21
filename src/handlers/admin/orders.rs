use actix_web::HttpRequest;
use actix_web::{get, put, web, HttpResponse, delete, Result};
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use crate::models::{Order, OrderItem};
use crate::db::DbPool;
use crate::schema::{order_items, orders};
use bigdecimal::BigDecimal;
use crate::utils::is_admin;

#[get("/api/admin/orders")]
pub async fn get_orders(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse> {
    
    if !is_admin(&req) {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let mut conn = pool.get().unwrap();
    let orders: Vec<Order> = orders::table.load(&mut conn).unwrap();
    Ok(HttpResponse::Ok().json(orders))
}

#[get("/api/admin/orders/{id}")]
pub async fn get_order(pool: web::Data<DbPool>, path: web::Path<i32>, req: HttpRequest) -> Result<HttpResponse> {
    if !is_admin(&req) {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let mut conn = pool.get().unwrap();
    let order: Order = orders::table.find(path.into_inner()).first(&mut conn).unwrap();
    Ok(HttpResponse::Ok().json(order))
}

#[derive(Deserialize, Serialize)]
pub struct OrderWithItems {
    pub order: Order,
    pub items: Vec<OrderItem>,
}

// get order items
#[get("/api/admin/orders/{id}/items")]
pub async fn get_order_with_items(pool: web::Data<DbPool>, path: web::Path<i32>, req: HttpRequest) -> Result<HttpResponse> {
    if !is_admin(&req) {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let mut conn = pool.get().unwrap();
    let order_id = path.into_inner();
    let order: Order = orders::table.find(order_id).first(&mut conn).unwrap();  
    let order_items: Vec<OrderItem> = order_items::table.filter(order_items::order_id.eq(order_id)).load(&mut conn).unwrap();
    Ok(HttpResponse::Ok().json(OrderWithItems { order, items: order_items }))
}  

#[derive(Deserialize)]
pub struct EditOrder {
    pub status: Option<String>,
    pub address: Option<String>,
    pub items: Option<Vec<EditOrderItem>>,
}

#[derive(Deserialize)]
pub struct EditOrderItem {
    pub product_id: Option<i32>,
    pub quantity: Option<i32>,
    pub price: Option<BigDecimal>,
}

#[put("/api/admin/orders/{id}")]
pub async fn update_order(pool: web::Data<DbPool>, path: web::Path<i32>, data: web::Json<EditOrder>, req: HttpRequest) -> Result<HttpResponse> {
    if !is_admin(&req) {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let mut conn = pool.get().unwrap(); 
    let order_id = path.into_inner();
    
    // Update order if fields are provided
    if data.status.is_some() || data.address.is_some() {
        if let Some(status) = &data.status {
            diesel::update(orders::table.find(order_id))
                .set(orders::status.eq(status))
                .execute(&mut conn)
                .unwrap();
        }
        if let Some(address) = &data.address {
            diesel::update(orders::table.find(order_id))
                .set(orders::address.eq(address))
                .execute(&mut conn)
                .unwrap();
        }
    }

    // Update order items if provided
    if let Some(items) = &data.items {
        for item in items {
            if let Some(product_id) = item.product_id {
                if let Some(quantity) = item.quantity {
                    diesel::update(order_items::table
                        .filter(order_items::order_id.eq(order_id))
                        .filter(order_items::product_id.eq(product_id)))
                        .set(order_items::quantity.eq(quantity))
                        .execute(&mut conn)
                        .unwrap();
                }
                if let Some(price) = &item.price {
                    diesel::update(order_items::table
                        .filter(order_items::order_id.eq(order_id))
                        .filter(order_items::product_id.eq(product_id)))
                        .set(order_items::price.eq(price))
                        .execute(&mut conn)
                        .unwrap();
                }
            }
        }

        // Recalculate total price
        let order_items = order_items::table
            .filter(order_items::order_id.eq(order_id))
            .load::<OrderItem>(&mut conn)
            .unwrap();

        let mut total = BigDecimal::from(0);
        for item in order_items {
            total += item.price * BigDecimal::from(item.quantity);
        }

        diesel::update(orders::table.find(order_id))
            .set(orders::total.eq(total))
            .execute(&mut conn)
            .unwrap();
    }

    let updated_order = orders::table.find(order_id).first::<Order>(&mut conn).unwrap();
    let order_items = order_items::table
        .filter(order_items::order_id.eq(order_id))
        .load::<OrderItem>(&mut conn)
        .unwrap();

    Ok(HttpResponse::Ok().json(OrderWithItems { 
        order: updated_order, 
        items: order_items 
    }))
}

#[delete("/api/admin/orders/{id}")]
pub async fn delete_order(pool: web::Data<DbPool>, path: web::Path<i32>, req: HttpRequest) -> Result<HttpResponse> {
    if !is_admin(&req) {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let mut conn = pool.get().unwrap();
    let order_id = path.into_inner();
    diesel::delete(orders::table.find(order_id)).execute(&mut conn).unwrap();
    diesel::delete(order_items::table.filter(order_items::order_id.eq(order_id))).execute(&mut conn).unwrap();
    Ok(HttpResponse::Ok().json(order_id))
}

// delete order item
#[delete("/api/admin/orders/{id}/items/{item_id}")]
pub async fn delete_order_item(pool: web::Data<DbPool>, path: web::Path<(i32, i32)>, req: HttpRequest) -> Result<HttpResponse> {
    if !is_admin(&req) {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let mut conn = pool.get().unwrap();
    let (order_id, item_id) = path.into_inner();
    diesel::delete(order_items::table.filter(order_items::order_id.eq(order_id)).filter(order_items::id.eq(item_id))).execute(&mut conn).unwrap();
    // Recalculate total price
    let order_items = order_items::table
        .filter(order_items::order_id.eq(order_id))
        .load::<OrderItem>(&mut conn)
        .unwrap();

    let mut total = BigDecimal::from(0);
    for item in order_items {
        total += item.price * BigDecimal::from(item.quantity);
    }

    diesel::update(orders::table.find(order_id))
        .set(orders::total.eq(total))
        .execute(&mut conn)
        .unwrap();

    Ok(HttpResponse::Ok().json(order_id))
}   