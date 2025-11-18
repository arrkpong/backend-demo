use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use sea_orm::DatabaseConnection;
use serde_json::json;
use std::collections::HashSet;
use std::sync::Mutex;

use crate::config::AppConfig;
use crate::services::user_service::find_user_by_id;
use crate::utils::decode_token;


#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/me")]
pub async fn profile(
    db: web::Data<DatabaseConnection>,
    config: web::Data<AppConfig>,
    revoked_tokens: web::Data<Mutex<HashSet<String>>>,
    req: HttpRequest,
) -> HttpResponse {
    let token = match req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer ").map(str::trim))
    {
        Some(token) => token,
        None => return HttpResponse::Unauthorized().body("Missing Authorization header"),
    };

    if revoked_tokens.lock().unwrap().contains(token) {
        return HttpResponse::Unauthorized().body("Token has been revoked");
    }

    let claims = match decode_token(&config.jwt_secret, token) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid or expired token"),
    };

    match find_user_by_id(db.get_ref(), claims.sub).await {
        Ok(Some(user)) => HttpResponse::Ok().json(json!({
            "id": user.id,
            "username": user.username,
        })),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("DB error when loading user: {}", e))
        }
    }
}
