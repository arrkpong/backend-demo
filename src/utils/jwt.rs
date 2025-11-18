use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub exp: usize,
}

/// Encode a JWT for the provided subject (typically a user ID).
pub fn encode_token(secret: &str, subject: i32) -> jsonwebtoken::errors::Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(30))
        .expect("failed to create expiration")
        .timestamp() as usize;

    let claims = TokenClaims {
        sub: subject,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

/// Decode and validate a JWT returning its claims.
pub fn decode_token(secret: &str, token: &str) -> jsonwebtoken::errors::Result<TokenClaims> {
    decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
