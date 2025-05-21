// === models.rs ===
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use crate::schema::*;

// USERS
#[derive(Queryable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub address: Option<String>,
    pub created_at: NaiveDateTime,
    pub role: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub role: Option<String>,
}

// PRODUCTS
#[derive(Queryable, Identifiable, Serialize)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub category: String,
    pub image_url: Option<String>,
    pub available: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub category: String,
    pub image_url: Option<String>,
    pub available: Option<bool>,
}

// FAVORITES
#[derive(Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(User)]
#[belongs_to(Product)]
#[table_name = "favorites"]
pub struct Favorite {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "favorites"]
pub struct NewFavorite {
    pub user_id: i32,
    pub product_id: i32,
}

// ORDERS
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[table_name = "orders"]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total: BigDecimal,
    pub status: String,
    pub address: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "orders"]
pub struct NewOrder {
    pub user_id: i32,
    pub total: BigDecimal,
    pub status: String,
    pub address: String,
}

// ORDER ITEMS
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Order)]
#[table_name = "order_items"]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub product_variant_id: i32,
    pub quantity: i32,
    pub price: BigDecimal,
}

#[derive(Insertable, Deserialize)]
#[table_name = "order_items"]
pub struct NewOrderItem {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: BigDecimal,
}

// REVIEWS
#[derive(Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(User)]
#[belongs_to(Product)]
#[table_name = "reviews"]
pub struct Review {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "reviews"]
pub struct NewReview {
    pub user_id: i32,
    pub product_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
}

// COMPLAINTS
#[derive(Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(User)]
#[belongs_to(Order)]
#[table_name = "complaints"]
pub struct Complaint {
    pub id: i32,
    pub order_id: i32,
    pub user_id: i32,
    pub reason: String,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "complaints"]
pub struct NewComplaint {
    pub order_id: i32,
    pub user_id: i32,
    pub reason: String,
    pub comment: Option<String>,
}

