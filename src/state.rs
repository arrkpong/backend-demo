use std::collections::HashSet;
use std::fmt::{self, Display};
use std::sync::Mutex;

use actix_web::{HttpRequest, HttpResponse, ResponseError, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::config::AppConfig;
use crate::utils::{TokenClaims, decode_token};

/// Shared state required by the handlers and middleware.
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: AppConfig,
    pub revoked_tokens: Mutex<HashSet<String>>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config: AppConfig) -> Self {
        Self {
            db,
            config,
            revoked_tokens: Mutex::new(HashSet::new()),
        }
    }

    pub fn validate_token(&self, token: &str) -> Result<TokenClaims, AuthError> {
        let claims =
            decode_token(&self.config.jwt_secret, token).map_err(|_| AuthError::InvalidToken)?;

        let revoked = self
            .revoked_tokens
            .lock()
            .map_err(|_| AuthError::LockError)?;

        if revoked.contains(token) {
            Err(AuthError::RevokedToken)
        } else {
            Ok(claims)
        }
    }

    pub fn revoke_token(&self, token: &str) -> Result<bool, AuthError> {
        let mut revoked = self
            .revoked_tokens
            .lock()
            .map_err(|_| AuthError::LockError)?;

        Ok(revoked.insert(token.to_owned()))
    }
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    RevokedToken,
    MissingHeader,
    LockError,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidToken => write!(f, "Invalid or expired token"),
            AuthError::RevokedToken => write!(f, "Token has been revoked"),
            AuthError::MissingHeader => write!(f, "Missing Authorization header"),
            AuthError::LockError => write!(f, "Internal lock error"),
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidToken | AuthError::RevokedToken => StatusCode::UNAUTHORIZED,
            AuthError::MissingHeader => StatusCode::BAD_REQUEST,
            AuthError::LockError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

/// Extracts the bearer token from the Authorization header.
pub fn bearer_token(req: &HttpRequest) -> Result<String, AuthError> {
    req.headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer ").map(str::trim))
        .map(str::to_string)
        .ok_or(AuthError::MissingHeader)
}
