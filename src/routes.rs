use std::convert::Infallible;
use std::sync::Arc;

use sqlx::PgPool;
use warp::Filter;

use crate::auth::routes::AuthRoutes;

use crate::errors::handle_rejection;

/// Combines all of the routes together
pub fn init(
    pool: Arc<PgPool>, // database pool
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    AuthRoutes::init(pool.clone())
        .with(warp::log("tallii-platform"))
        .recover(handle_rejection)
}
