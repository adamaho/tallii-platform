use std::sync::Arc;

use argon2::{self, Config};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use super::db::{User, UserResponse};
use super::token::{Claims, TokenResponse};
use crate::errors::TalliiError;
use crate::ResponseResult;

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
    user: UserResponse,
}

/// Checks the users email and password and responds with an access token
pub async fn login(payload: LoginPayload, pool: Arc<PgPool>) -> ResponseResult<impl warp::Reply> {
    // validate the request payload
    payload
        .validate()
        .map_err(|e| warp::reject::custom(TalliiError::ValidationError(e.to_string())))?;

    // get the user from the database
    let user = User::get_by_email_option(&*pool, &payload.email).await?;

    match user {
        Some(user) => {
            // check to make sure the passwords are the same, if they arent, return an error
            let password = payload.password.as_bytes();
            let hash = user.password;
            let matches = argon2::verify_encoded(&hash, password).unwrap();

            if !matches {
                return Err(warp::reject::custom(TalliiError::Unauthorized));
            }

            // create a new jwt
            let access_token = Claims::generate_jwt(&user.email, &user.user_id)
                .map_err(|e| warp::reject::custom(e))?;

            // create response
            let response = LoginResponse {
                access_token,
                user: UserResponse {
                    user_id: user.user_id,
                    username: user.username,
                    email: user.email,
                    created_at: user.created_at,
                },
            };

            // respond with the access and refresh tokens
            return Ok(warp::reply::json(&response));
        }
        None => Err(warp::reject::custom(TalliiError::Unauthorized)),
    }
}
