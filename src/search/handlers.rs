use std::sync::Arc;
use std::collections::HashMap;

use jsonwebtoken::TokenData;
use serde::Serialize;
use sqlx::PgPool;

use crate::auth::db::{User, UserResponse};
use crate::auth::token::Claims;
use crate::errors::TalliiError;
use crate::ResponseResult;

#[derive(Serialize)]
pub struct SearchResults {
    pub users: Vec<UserResponse>
}

/// searches for users right now. more to come in the future
pub async fn search(
    params: HashMap<String, String>,
    pool: Arc<PgPool>,
    _token: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    match params.get(&String::from("query")) {
        Some(query) => {
            let users = User::search_users(&pool, query).await?;

            let response = SearchResults {
                users
            };

            return Ok(warp::reply::json(&response));
        }
        None => {
            return Err(warp::reject::custom(TalliiError::BadRequest(String::from("Invalid query parameters"))));
        }
    }
}