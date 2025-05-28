use crate::db::DbPool;
use crate::handlers::auth::Claims;
use crate::models::{NewReview, Review};
use crate::schema::reviews;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateReview {
    pub product_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
}

#[post("/api/user/reviews")]
pub async fn create_review(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    review: web::Json<CreateReview>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    let new_review = NewReview {
        user_id: claims.sub,
        product_id: review.product_id,
        rating: review.rating,
        comment: review.comment.clone(),
    };

    diesel::insert_into(reviews::table)
        .values(&new_review)
        .execute(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(new_review)
}

#[get("/api/user/reviews/{product_id}")]
pub async fn get_reviews_by_product(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let reviews = reviews::table
        .filter(reviews::product_id.eq(path.into_inner()))
        .load::<Review>(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(reviews)
}
