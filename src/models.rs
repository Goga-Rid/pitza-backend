use crate::schema::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub role: Option<String>,
}

// PRODUCTS
#[derive(Queryable, Identifiable, Serialize)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub category: String,
    pub image_url: Option<String>,
    pub available: bool,
    pub weight: Option<i32>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub category: String,
    pub image_url: Option<String>,
    pub available: Option<bool>,
    pub weight: Option<i32>,
}

// FAVORITES
#[derive(Queryable, Identifiable, Associations, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = favorites)]
pub struct Favorite {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = favorites)]
pub struct NewFavorite {
    pub user_id: i32,
    pub product_id: i32,
}

// ORDERS
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total: BigDecimal,
    pub status: String,
    pub address: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub total: BigDecimal,
    pub status: String,
    pub address: String,
}

// ORDER ITEMS
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Order))]
#[diesel(table_name = order_items)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub product_variant_id: i32,
    pub quantity: i32,
    pub price: BigDecimal,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = order_items)]
pub struct NewOrderItem {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: BigDecimal,
}

// REVIEWS
#[derive(Queryable, Identifiable, Associations, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = reviews)]
pub struct Review {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = reviews)]
pub struct NewReview {
    pub user_id: i32,
    pub product_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
}

// COMPLAINTS
#[derive(Queryable, Identifiable, Associations, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Order))]
#[diesel(table_name = complaints)]
pub struct Complaint {
    pub id: i32,
    pub order_id: i32,
    pub user_id: i32,
    pub reason: String,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = complaints)]
pub struct NewComplaint {
    pub order_id: i32,
    pub user_id: i32,
    pub reason: String,
    pub comment: Option<String>,
}
