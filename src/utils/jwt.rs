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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_and_decode_roundtrip() {
        let secret = "test-secret";
        let subject = 42;

        let token = encode_token(secret, subject).expect("token should encode");
        let claims = decode_token(secret, &token).expect("token should decode");

        assert_eq!(claims.sub, subject);
        assert!(claims.exp > Utc::now().timestamp() as usize);
    }

    #[test]
    fn decode_fails_with_wrong_secret() {
        let token = encode_token("one-secret", 1).expect("token should encode");
        assert!(decode_token("different-secret", &token).is_err());
    }
}
