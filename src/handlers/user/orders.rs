use crate::db::DbPool;
use crate::handlers::auth::Claims;
use crate::models::{NewOrder, NewOrderItem};
use crate::models::{Order, OrderItem, User};
use crate::schema::order_items;
use crate::schema::{orders, products, users};
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;

#[get("/api/user/orders")]
pub async fn get_orders(req: HttpRequest, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    // Get orders
    let orders: Vec<Order> = orders::table
        .filter(orders::user_id.eq(claims.sub))
        .load(&mut conn)
        .unwrap();

    // Get item counts for each order
    let orders_with_items: Vec<serde_json::Value> = orders
        .into_iter()
        .map(|order| {
            let item_count: i64 = order_items::table
                .filter(order_items::order_id.eq(order.id))
                .count()
                .get_result(&mut conn)
                .unwrap_or(0);

            serde_json::json!({
                "order": order,
                "item_count": item_count
            })
        })
        .collect();

    HttpResponse::Ok().json(orders_with_items)
}

#[get("/api/user/orders/{id}")]
pub async fn get_order_with_items_by_id(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let order: Order = orders::table
        .filter(orders::id.eq(path.into_inner()))
        .filter(orders::user_id.eq(claims.sub))
        .first(&mut conn)
        .unwrap();
    let order_items: Vec<OrderItem> = order_items::table
        .filter(order_items::order_id.eq(order.id))
        .load(&mut conn)
        .unwrap();
    HttpResponse::Ok().json(json!({
        "order": order,
        "order_items": order_items
    }))
}
#[derive(Deserialize)]
pub struct CreateOrderItem {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub items: Vec<CreateOrderItem>,
}

#[post("/api/user/orders")]
pub async fn create_order(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    order_data: web::Json<CreateOrderRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    // Get user's address
    let user = users::table
        .find(claims.sub)
        .first::<User>(&mut conn)
        .expect("Error loading user");

    if user.address.is_none() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "User address is not set"
        }));
    }

    // Calculate total price
    let mut total = BigDecimal::from(0);
    for item in &order_data.items {
        let product_price: BigDecimal = products::table
            .find(item.product_id)
            .select(products::price)
            .first(&mut conn)
            .expect("Error loading product price");

        total += product_price * BigDecimal::from(item.quantity);
    }

    // Create order
    let new_order = NewOrder {
        user_id: claims.sub,
        total,
        status: "оформлен".to_string(),
        address: user.address.unwrap(),
    };

    let order = diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result::<Order>(&mut conn)
        .expect("Error saving order");

    // Create order items
    let order_items: Vec<NewOrderItem> = order_data
        .items
        .iter()
        .map(|item| {
            let product_price: BigDecimal = products::table
                .find(item.product_id)
                .select(products::price)
                .first(&mut conn)
                .expect("Error loading product price");

            NewOrderItem {
                order_id: order.id,
                product_id: item.product_id,
                quantity: item.quantity,
                price: product_price,
            }
        })
        .collect();

    diesel::insert_into(order_items::table)
        .values(&order_items)
        .execute(&mut conn)
        .expect("Error saving order items");

    HttpResponse::Ok().json(order)
}
