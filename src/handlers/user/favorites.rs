use crate::db::DbPool;
use crate::handlers::auth::Claims;
use crate::models::{Favorite, NewFavorite};
use crate::schema::favorites;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateFavorite {
    pub product_id: i32,
}

#[post("/api/user/favorites")]
pub async fn create_favorite(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    favorite: web::Json<CreateFavorite>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .expect("Claims not found in request extensions");

    let new_favorite = NewFavorite {
        user_id: claims.sub,
        product_id: favorite.product_id,
    };

    diesel::insert_into(favorites::table)
        .values(&new_favorite)
        .execute(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(new_favorite)
}

#[get("/api/user/favorites")]
pub async fn get_favorites(req: HttpRequest, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .expect("Claims not found in request extensions");

    let favorites = favorites::table
        .filter(favorites::user_id.eq(claims.sub))
        .load::<Favorite>(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(favorites)
}

#[delete("/api/user/favorites/{id}")]
pub async fn delete_favorite(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .expect("Claims not found in request extensions");

    let product_ids = path.into_inner();
    println!(
        "Attempting to delete favorite with product_id: {}",
        product_ids
    );

    let favorite = favorites::table
        .filter(favorites::product_id.eq(product_ids))
        .filter(favorites::user_id.eq(claims.sub))
        .first::<Favorite>(&mut conn)
        .optional()
        .unwrap();

    if favorite.is_none() {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Favorite not found"
        }));
    }

    let deleted = diesel::delete(
        favorites::table
            .filter(favorites::product_id.eq(product_ids))
            .filter(favorites::user_id.eq(claims.sub)),
    )
    .execute(&mut conn)
    .unwrap();

    println!("Deleted {} favorites", deleted);

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Favorite deleted successfully"
    }))
}
