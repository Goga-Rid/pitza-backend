use actix_web::{get, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier, PasswordHasher};
use argon2::password_hash::SaltString;
use crate::models::User;
use crate::schema::users;
use diesel::prelude::*;
use crate::handlers::auth::Claims;
use crate::db::DbPool;
use serde::Deserialize;

#[get("/api/me")]
pub async fn get_me(req: HttpRequest, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let user = users::table.find(claims.sub)
        .first::<User>(&mut conn)
        .unwrap();
    HttpResponse::Ok().json(user)
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub address: Option<String>,
    pub old_password: Option<String>,
    pub new_password: Option<String>,
}

// update user
#[put("/api/me")]
pub async fn update_user(req: HttpRequest, pool: web::Data<DbPool>, data: web::Json<UpdateUser>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    let user = users::table.find(claims.sub)
        .first::<User>(&mut conn)
        .unwrap();
    
    if let Some(name) = &data.name {
        diesel::update(users::table.find(claims.sub))
            .set(users::name.eq(name))
            .execute(&mut conn)
            .unwrap();
    }
    if let Some(address) = &data.address {
        diesel::update(users::table.find(claims.sub))
            .set(users::address.eq(address))
            .execute(&mut conn)
            .unwrap();
    }
    if let Some(new_password) = &data.new_password {
        if let Some(old_password) = &data.old_password {
            let parsed_hash = PasswordHash::new(&user.password).expect("Error parsing password hash");
            let argon2 = Argon2::default();

            if argon2
                .verify_password(old_password.as_bytes(), &parsed_hash)
                .is_err()
            {
                return HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid password"
                }));
            }

            let salt = SaltString::generate(&mut rand::thread_rng());
            let argon2 = Argon2::default();
            let password_hash = argon2
                .hash_password(new_password.as_bytes(), &salt)
                .expect("Error hashing password")
                .to_string();

            diesel::update(users::table.find(claims.sub))
                .set(users::password.eq(password_hash))
                .execute(&mut conn)
                .unwrap();
        }
    }
    
    let updated_user = users::table.find(claims.sub)
        .first::<User>(&mut conn)
        .unwrap();
    
    HttpResponse::Ok().json(updated_user)   
}
