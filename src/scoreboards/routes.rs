use std::sync::Arc;

use jsonwebtoken::TokenData;
use sqlx::PgPool;
use warp::Filter;

use super::handlers;
use crate::users::token::Claims;
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
            .or(get_user_scoreboards(pool.clone()))
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
        .and_then(move |pool: Arc<PgPool>, token: TokenData<Claims>| {
            handlers::get_user_scoreboards(token.claims.sub, pool)
        })
}

/// gets all scoreboards for the matching user
pub fn get_user_scoreboards(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "users" / i32 / "scoreboards")
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(
            move |user_id: i32, pool: Arc<PgPool>, _token: TokenData<Claims>| {
                handlers::get_user_scoreboards(user_id, pool)
            },
        )
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
