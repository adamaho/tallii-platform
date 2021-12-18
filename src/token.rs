use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, dangerous_insecure_decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::TalliiError;
use crate::Result;

/// Represents the contents of a jwt
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub email: String,
    pub exp: i64,
}

impl Claims {
    /// Decodes and verifies the provided token to the Token struct
    pub fn verify_jwt(token: String) -> Result<jsonwebtoken::TokenData<Claims>> {

        // get secret from env
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");

        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(&secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(claims) => Ok(claims),
            Err(_) => Err(TalliiError::InvalidToken),
        }
    }

    /// Decodes the provided token to the Token struct with no verification
    pub fn decode_jwt(token: String) -> Result<jsonwebtoken::TokenData<Claims>> {

        match dangerous_insecure_decode::<Claims>(
            &token
        ) {
            Ok(claims) => Ok(claims),
            Err(_) => Err(TalliiError::InvalidToken),
        }
    }

    /// Encodes the provided token struct to a string
    pub fn generate_jwt(email: &str, user_id: &i32) -> Result<String> {
        // get secret from env
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");

        // expire the token in 5 minutes
        let now = Utc::now() + Duration::minutes(15);

        // create the claims
        let claims = Claims {
            sub: user_id.clone(),
            email: email.to_string(),
            exp: now.timestamp(),
        };

        // encode the jwt
        match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&secret.as_bytes()),
        ) {
            Ok(token) => Ok(token),
            Err(e) => Err(TalliiError::InternalServerError(e.to_string())),
        }
    }
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String
}
