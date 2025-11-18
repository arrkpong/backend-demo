use actix_web::{HttpRequest, HttpResponse, ResponseError, post, web};
use serde::Deserialize;
use serde_json::json;

use crate::services::user_service::{create_user, find_user_by_username};
use crate::state::{self, AppState};
use crate::utils::{encode_token, hash_password, verify_password};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

#[post("/auth/login")]
pub async fn login(
    state: web::Data<AppState>,
    login_payload: web::Json<LoginRequest>,
) -> HttpResponse {
    match find_user_by_username(&state.db, &login_payload.username).await {
        Ok(Some(user)) => {
            if verify_password(&user.password, &login_payload.password).is_err() {
                HttpResponse::Unauthorized().body("Invalid password.")
            } else {
                match encode_token(&state.config.jwt_secret, user.id) {
                    Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
                    Err(e) => HttpResponse::InternalServerError()
                        .body(format!("JWT encoding failed: {}", e)),
                }
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid username or password."),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("DB error on fetching user: {}", e))
        }
    }
}

#[post("/auth/register")]
pub async fn register(
    state: web::Data<AppState>,
    register_payload: web::Json<RegisterRequest>,
) -> HttpResponse {
    match find_user_by_username(&state.db, &register_payload.username).await {
        Ok(Some(_)) => return HttpResponse::BadRequest().body("Username already exists."),
        Ok(None) => {}
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("DB error on checking username: {}", e));
        }
    }

    let hashed_password = match hash_password(&register_payload.password) {
        Ok(hash) => hash,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Password hashing failed: {}", e));
        }
    };

    match create_user(
        &state.db,
        register_payload.username.clone(),
        hashed_password,
    )
    .await
    {
        Ok(created_user) => match encode_token(&state.config.jwt_secret, created_user.id) {
            Ok(token) => HttpResponse::Ok().json(json!({
                "token": token,
                "user": {
                    "id": created_user.id,
                    "username": created_user.username,
                }
            })),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("JWT encoding failed: {}", e))
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("DB error on insert user: {}", e))
        }
    }
}

#[post("/auth/logout")]
pub async fn logout(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    let token = match state::bearer_token(&req) {
        Ok(token) => token,
        Err(err) => return err.error_response(),
    };

    match state.revoke_token(&token) {
        Ok(true) => HttpResponse::Ok().body("Logged out successfully."),
        Ok(false) => HttpResponse::BadRequest().body("Token already revoked"),
        Err(err) => err.error_response(),
    }
}
