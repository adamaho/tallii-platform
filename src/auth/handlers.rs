use std::sync::Arc;

use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;
use argon2::{self, Config};

use crate::auth::db::User;
use crate::auth::token::{Claims, TokenResponse};
use crate::errors::TalliiError;
use crate::ResponseResult;

/// ------------------------------------
/// Log in
/// ------------------------------------
#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
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
            let access_token =
                Claims::generate_jwt(&user.email, &user.user_id).map_err(|e| warp::reject::custom(e))?;
        
            // create response
            let response = TokenResponse { access_token };
        
            // respond with the access and refresh tokens
            return Ok(warp::reply::json(&response))
        }
        None => {
            Err(warp::reject::custom(TalliiError::Unauthorized))
        }
    }
}
