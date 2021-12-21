use std::sync::Arc;

use jsonwebtoken::TokenData;
use sqlx::PgPool;

use crate::auth::token::Claims;
use crate::ResponseResult;

use super::db;

/// creates a scoreboard
pub async fn create_scoreboard(payload: db::CreateScoreboardPayload, pool: Arc<PgPool>, token: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {
    let scoreboard = db::Scoreboard::create_scoreboard(&pool, &payload, &token.claims.sub).await?;
    Ok(warp::reply::json(&scoreboard))
}

/// gets a single scoreboard
pub async fn get_scoreboard(scoreboard_id: i32, pool: Arc<PgPool>, _token: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {
    let scoreboard = db::Scoreboard::get_scoreboard(&pool, &scoreboard_id).await?;
    Ok(warp::reply::json(&scoreboard))
}

/// gets all scoreboards
pub async fn get_scoreboards( pool: Arc<PgPool>, _token: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {
    let scoreboards = db::Scoreboard::get_scoreboards(&pool).await?;
    Ok(warp::reply::json(&scoreboards))
}
