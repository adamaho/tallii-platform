use std::sync::Arc;

use sqlx::PgPool;
use warp::Filter;

use super::handlers;
use crate::wrappers::{with_auth, with_pool};

pub struct TeamRoutes;

impl TeamRoutes {
    /// Init the team routes
    pub fn init(
        pool: Arc<PgPool>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        get_teams(pool.clone()).or(get_team(pool.clone()))
    }
}

/// gets a single
pub fn get_team(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "teams" / i32)
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::get_team)
}

/// gets all teams
pub fn get_teams(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "teams")
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::get_teams)
}
