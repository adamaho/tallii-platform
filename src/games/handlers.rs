use std::sync::Arc;

use jsonwebtoken::TokenData;
use sqlx::PgPool;

use crate::auth::token::Claims;
use crate::ResponseResult;

use super::db;

/// creates a game
pub async fn create_game(payload: db::CreateGamePayload, pool: Arc<PgPool>, _claims: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {
    let game = db::Game::create_game(&pool, &payload).await?;
    Ok(warp::reply::json(&game))
}

/// gets a single game
pub async fn get_game(game_id: i32, pool: Arc<PgPool>, _claims: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {
    let game = db::Game::get_game(&pool, &game_id).await?;
    Ok(warp::reply::json(&game))
}

/// gets all games
pub async fn get_games( pool: Arc<PgPool>, _claims: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {
    let games = db::Game::get_games(&pool).await?;
    Ok(warp::reply::json(&games))
}
