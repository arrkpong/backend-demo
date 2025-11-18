use actix_web::{Error, HttpMessage, dev::ServiceRequest, error::ErrorUnauthorized, web};

use crate::config::AppConfig;
use crate::utils::{TokenClaims, decode_token};

/// Simple middleware helper that can be used once JWT support is added.
#[allow(dead_code)]
pub fn ensure_auth_header(req: &ServiceRequest) -> Result<TokenClaims, Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer ").map(str::trim))
        .ok_or_else(|| ErrorUnauthorized("Missing Authorization header"))?;

    let config = req
        .app_data::<web::Data<AppConfig>>()
        .ok_or_else(|| ErrorUnauthorized("Missing configuration"))?;

    let claims = decode_token(&config.jwt_secret, token)
        .map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;

    req.extensions_mut().insert(claims.clone());

    Ok(claims)
}
