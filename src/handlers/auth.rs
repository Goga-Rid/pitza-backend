use actix_web::{web, HttpResponse, Responder, HttpRequest, post, get};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use argon2::{Argon2, PasswordHash, PasswordVerifier, PasswordHasher};
use argon2::password_hash::SaltString;
use chrono::{Utc, Duration};
use std::env;

use crate::models::{User, NewUser};
use crate::schema::users::dsl::*;
use crate::DbPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user id
    pub exp: i64, // expiration time
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
}

#[post("/api/register")]
pub async fn register(
    pool: web::Data<DbPool>,
    user_data: web::Json<RegisterUser>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // Check if user already exists
    let existing_user = users
        .filter(email.eq(&user_data.email))
        .first::<User>(&mut conn)
        .optional()
        .expect("Error checking for existing user");

    if existing_user.is_some() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "User with this email already exists"
        }));
    }

    // Hash password with Argon2
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user_data.password.as_bytes(), &salt)
        .expect("Error hashing password")
        .to_string();

    // Create new user
    let new_user = NewUser {
        email: user_data.email.clone(),
        password: password_hash,
        name: user_data.name.clone(),
        role: Some("user".to_string()),
    };

    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut conn)
        .expect("Error saving new user");

    // Generate JWT token
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id,
        exp: expiration,
        role: user.role,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Error creating token");

    HttpResponse::Ok().json(AuthResponse {
        token,
        user_id: user.id,
    })
}

#[post("/api/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    login_data: web::Json<LoginUser>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user = match users
        .filter(email.eq(&login_data.email))
        .first::<User>(&mut conn)
        .optional()
        .expect("Error finding user")
    {
        Some(user) => user,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid email or password"
            }));
        }
    };

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password).expect("Error parsing password hash");
    let argon2 = Argon2::default();

    if argon2
        .verify_password(login_data.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid email or password"
        }));
    }

    // Generate JWT token
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id,
        exp: expiration,
        role: user.role,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).unwrap();
    
    HttpResponse::Ok().json(AuthResponse {
        token,
        user_id: user.id,
    })
}

#[get("/validate")]
pub async fn validate_token(req: HttpRequest) -> impl Responder {
    let auth_header = req.headers().get("Authorization");
    
    if auth_header.is_none() {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "No token provided"
        }));
    }

    let token = auth_header.unwrap().to_str().unwrap().replace("Bearer ", "");
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "valid"
        })),
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid token"
        })),
    }
}
