use actix_web::{HttpRequest, HttpResponse, ResponseError, post, web};
use serde::Deserialize;
use serde_json::json;

use crate::services::user_service::{create_user, find_user_by_username};
use crate::state::{self, AppState};
use crate::utils::encode_token;

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
            if user.password != login_payload.password {
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

    match create_user(
        &state.db,
        register_payload.username.clone(),
        register_payload.password.clone(),
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

#[cfg(test)]
mod tests {
    use actix_web::{App, http::StatusCode, test, web};
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use serde_json::Value;

    use crate::{
        config::AppConfig,
        models::user::Model as UserModel,
        state::AppState,
        utils::encode_token,
    };

    use super::*;

    fn test_config() -> AppConfig {
        AppConfig {
            database_url: "postgres://localhost:5432/postgres".to_string(),
            bind_address: "127.0.0.1:8080".to_string(),
            jwt_secret: "test-secret".to_string(),
        }
    }

    fn mock_state(
        query_results: Vec<Vec<UserModel>>,
        exec_results: Vec<MockExecResult>,
    ) -> web::Data<AppState> {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(query_results)
            .append_exec_results(exec_results)
            .into_connection();

        web::Data::new(AppState::new(db, test_config()))
    }

    #[actix_web::test]
    async fn login_returns_token_on_valid_credentials() {
        let user = UserModel {
            id: 1,
            username: "alice".into(),
            password: "secret".into(),
        };
        let state = mock_state(vec![vec![user]], vec![]);

        let app = test::init_service(App::new().app_data(state.clone()).service(login)).await;
        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&serde_json::json!({"username": "alice", "password": "secret"}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Value = test::read_body_json(resp).await;
        assert!(body.get("token").and_then(Value::as_str).is_some());
    }

    #[actix_web::test]
    async fn login_rejects_wrong_password() {
        let user = UserModel {
            id: 1,
            username: "alice".into(),
            password: "secret".into(),
        };
        let state = mock_state(vec![vec![user]], vec![]);

        let app = test::init_service(App::new().app_data(state.clone()).service(login)).await;
        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&serde_json::json!({"username": "alice", "password": "wrong"}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn login_rejects_unknown_user() {
        let state = mock_state(vec![vec![]], vec![]);

        let app = test::init_service(App::new().app_data(state.clone()).service(login)).await;
        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&serde_json::json!({"username": "ghost", "password": "whatever"}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn register_creates_user_when_username_free() {
        let created = UserModel {
            id: 10,
            username: "newuser".into(),
            password: "pw".into(),
        };
        let state = mock_state(
            vec![
                vec![],          // check for existing username
                vec![created.clone()], // insert returning created row
            ],
            vec![MockExecResult {
                last_insert_id: 10,
                rows_affected: 1,
            }],
        );

        let app = test::init_service(App::new().app_data(state.clone()).service(register)).await;
        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&serde_json::json!({"username": "newuser", "password": "pw"}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["user"]["id"], 10);
        assert_eq!(body["user"]["username"], "newuser");
    }

    #[actix_web::test]
    async fn register_rejects_duplicate_username() {
        let existing = UserModel {
            id: 1,
            username: "taken".into(),
            password: "pw".into(),
        };
        let state = mock_state(vec![vec![existing]], vec![]);

        let app = test::init_service(App::new().app_data(state.clone()).service(register)).await;
        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&serde_json::json!({"username": "taken", "password": "pw"}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn logout_revokes_token() {
        let state = mock_state(vec![], vec![]);
        let token =
            encode_token(&state.config.jwt_secret, 3).expect("should encode test token successfully");

        let app = test::init_service(App::new().app_data(state.clone()).service(logout)).await;
        let req = test::TestRequest::post()
            .uri("/auth/logout")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn logout_requires_authorization_header() {
        let state = mock_state(vec![], vec![]);

        let app = test::init_service(App::new().app_data(state.clone()).service(logout)).await;
        let req = test::TestRequest::post().uri("/auth/logout").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
