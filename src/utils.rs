use actix_web::{HttpMessage, HttpRequest};

use crate::handlers::auth::Claims;


pub fn is_admin(req: &HttpRequest) -> bool {
    req.extensions()
        .get::<Claims>()
        .map(|claims| claims.role == "admin")
        .unwrap_or(false)
}