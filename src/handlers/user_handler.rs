use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError, get, web};
use serde_json::json;

use crate::state::{self, AppState};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to home page.")
}

#[get("/me")]
pub async fn profile(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let token = match state::bearer_token(&req) {
        Ok(token) => token,
        Err(err) => return err.error_response(),
    };

    let claims = match state.validate_token(&token) {
        Ok(claims) => claims,
        Err(err) => return err.error_response(),
    };

    match crate::services::user_service::find_user_by_id(&state.db, claims.sub).await {
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
