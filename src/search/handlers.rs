use std::sync::Arc;
use std::collections::HashMap;

use jsonwebtoken::TokenData;
use sqlx::PgPool;

use crate::auth::db::User;
use crate::auth::token::Claims;
use crate::errors::TalliiError;
use crate::ResponseResult;

/// searches for users right now. more to come in the future
pub async fn search(
    params: HashMap<String, String>,
    pool: Arc<PgPool>,
    _token: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    match params.get(&String::from("query")) {
        Some(query) => {
            let results = User::search_users(&pool, query).await?;

            return Ok(warp::reply::json(&results));
        }
        None => {
            return Err(warp::reject::custom(TalliiError::BadRequest(String::from("Invalid query parameters"))));
        }
    }
}