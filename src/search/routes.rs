use std::sync::Arc;
use std::collections::HashMap;

use warp::Filter;
use sqlx::PgPool;

use crate::wrappers::{with_auth, with_pool};

use super::handlers;

pub struct SearchRoutes;

impl SearchRoutes {
    /// Init the scoreboard routes
    pub fn init(
        pool: Arc<PgPool>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        search(pool.clone())
    }
}

/// GET /v1/search
pub fn search(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "search")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::search)
}