use std::sync::Arc;

use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use super::db::{User, UserResponse};
use super::token::Claims;

use crate::config::Config;
use crate::errors::TalliiError;
use crate::ResponseResult;

//////////////////////////////////////////////////
/// get user profile
//////////////////////////////////////////////////
pub async fn get_me(pool: Arc<PgPool>, token: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {
    let user = User::get_by_user_id(&pool, &token.claims.sub).await?;

    let response = UserResponse {
        user_id: user.user_id,
        email: user.email,
        username: user.username,
        avatar_background: user.avatar_background,
        avatar_emoji: user.avatar_emoji,
        created_at: user.created_at
    };

    Ok(warp::reply::json(&response))
}

//////////////////////////////////////////////////
/// Log user in
//////////////////////////////////////////////////
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
                    avatar_background: user.avatar_background,
                    avatar_emoji: user.avatar_emoji,
                    created_at: user.created_at,
                },
            };

            // respond with the access and refresh tokens
            return Ok(warp::reply::json(&response));
        }
        None => Err(warp::reject::custom(TalliiError::Unauthorized)),
    }
}

//////////////////////////////////////////////////
/// sign user up
//////////////////////////////////////////////////
#[derive(Deserialize, Validate)]
pub struct SignupPayload {
    #[validate(length(min = 3))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn signup(
    payload: SignupPayload,
    pool: Arc<PgPool>,
    config: Config,
) -> ResponseResult<impl warp::Reply> {
    // validate the request payload
    payload
        .validate()
        .map_err(|e| warp::reject::custom(TalliiError::ValidationError(e.to_string())))?;

    // check if user with email exists
    let user = User::get_by_email_option(&*pool, &payload.email).await?;

    // if the user exists, return an error denoting that the email already exists
    if user.is_some() {
        return Err(warp::reject::custom(TalliiError::UserEmailTaken));
    }

    // create the hashed password
    let argon_config = argon2::Config::default();
    let hash = argon2::hash_encoded(
        payload.password.as_bytes(),
        &config.salt.as_bytes(),
        &argon_config,
    )
    .unwrap();

    // insert the user
    let created_user = User::create_user(&*pool, &payload.username, &payload.email, &hash).await?;

    // create the access token
    let access_token = Claims::generate_jwt(&created_user.email, &created_user.user_id)
        .map_err(|e| warp::reject::custom(e))?;

    // create response
    let response = LoginResponse {
        access_token,
        user: UserResponse {
            user_id: created_user.user_id,
            username: created_user.username,
            email: created_user.email,
            avatar_background: created_user.avatar_background,
            avatar_emoji: created_user.avatar_emoji,
            created_at: created_user.created_at,
        },
    };

    Ok(warp::reply::json(&response))
}


//////////////////////////////////////////////////
/// update user profile
//////////////////////////////////////////////////
#[derive(Deserialize)]
pub struct UpdateMeRequestPayload {
    pub username: String,
    pub avatar_background: String,
    pub avatar_emoji: String
}

pub async fn update_me(payload: UpdateMeRequestPayload, pool: Arc<PgPool>, token: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {
    let user = User::update_user(&pool, &token.claims.sub, &payload.username, &payload.avatar_background, &payload.avatar_emoji).await?;

    let response = UserResponse {
        user_id: user.user_id,
        email: user.email,
        username: user.username,
        avatar_background: user.avatar_background,
        avatar_emoji: user.avatar_emoji,
        created_at: user.created_at
    };

    Ok(warp::reply::json(&response))
}