use std::sync::Arc;

use sqlx::PgPool;
use warp::Filter;

use super::handlers;
use crate::wrappers::{with_auth, with_pool};

pub struct ScoreboardRoutes;

impl ScoreboardRoutes {
    /// Init the scoreboard routes
    pub fn init(
        pool: Arc<PgPool>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        create_scoreboard(pool.clone()).or(get_scoreboards(pool.clone())).or(get_scoreboard(pool.clone()))
    }
}

/// creates a scoreboard
pub fn create_scoreboard(
    pool: Arc<PgPool>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "scoreboards")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::create_scoreboard)
}

/// gets a single
pub fn get_scoreboard(
    pool: Arc<PgPool>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "scoreboards" / i32)
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::get_scoreboard)
}

/// gets all scoreboards
pub fn get_scoreboards(
    pool: Arc<PgPool>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "scoreboards")
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::get_scoreboards)
}