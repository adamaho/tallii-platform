use std::sync::Arc;

use jsonwebtoken::TokenData;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::config::Config;
use crate::db::{User};
use crate::errors::TalliiError;
use crate::token::{Claims, TokenResponse};
use crate::ResponseResult;

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    email: String,
}

/// Handle logging in
pub async fn login(
    payload: LoginPayload,
    pool: Arc<PgPool>,
) -> ResponseResult<impl warp::Reply> {
    // validate the request payload
    payload
        .validate()
        .map_err(|e| warp::reject::custom(TalliiError::ValidationError(e.to_string())))?;

    // get the user from the database
    let user = User::get_or_create(&*pool, &payload.email).await?;

    // create a new jwt
    let access_token =
        Claims::generate_jwt(&user.email, &user.user_id).map_err(|e| warp::reject::custom(e))?;

    // create response
    let response = TokenResponse {
        access_token
    };

    // respond with the access and refresh tokens
    Ok(warp::reply::json(&response))
}