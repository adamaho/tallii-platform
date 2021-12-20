use std::sync::Arc;

use sqlx::PgPool;
use warp::Filter;

use super::handlers;
use crate::wrappers::{with_auth, with_pool};

pub struct GameRoutes;

impl GameRoutes {
    /// Init the game routes
    pub fn init(
        pool: Arc<PgPool>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        create_game(pool.clone()).or(get_games(pool.clone())).or(get_game(pool.clone()))
    }
}

/// creates a game
pub fn create_game(
    pool: Arc<PgPool>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "games")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::create_game)
}

/// gets a single
pub fn get_game(
    pool: Arc<PgPool>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "games" / i32)
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::get_game)
}

/// gets all games
pub fn get_games(
    pool: Arc<PgPool>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "games")
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::get_games)
}