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
        create_scoreboard(pool.clone())
            .or(get_me_scoreboards(pool.clone()))
            .or(get_scoreboard(pool.clone()))
            .or(delete_scoreboard(pool.clone()))
    }
}

/// creates a scoreboard
pub fn create_scoreboard(
    pool: Arc<PgPool>,
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
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "scoreboards" / i32)
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::get_scoreboard)
}

/// gets all scoreboards for the currently logged in user
pub fn get_me_scoreboards(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "me" / "scoreboards")
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::get_me_scoreboards)
}


/// deletes the provided user
pub fn delete_scoreboard(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "scoreboards" / i32)
        .and(warp::delete())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::delete_scoreboard)
}
