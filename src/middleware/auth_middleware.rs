use actix_web::{Error, HttpMessage, dev::ServiceRequest, error::ErrorUnauthorized, web};

use crate::state::{self, AppState};
use crate::utils::TokenClaims;

/// Simple middleware helper that can be used once JWT support is added.
#[allow(dead_code)]
pub fn ensure_auth_header(req: &ServiceRequest) -> Result<TokenClaims, Error> {
    let token =
        state::bearer_token(req.request()).map_err(|err| ErrorUnauthorized(err.to_string()))?;

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ErrorUnauthorized("Missing application state"))?;

    let claims = state.validate_token(&token).map_err(Error::from)?;

    req.extensions_mut().insert(claims.clone());

    Ok(claims)
}
