use std::convert::Infallible;
use std::sync::Arc;

use sqlx::PgPool;
use warp::Filter;

use crate::config::Config;
use crate::errors::handle_rejection;

use crate::scoreboards::routes::ScoreboardRoutes;
use crate::search::routes::SearchRoutes;
use crate::teams::routes::TeamRoutes;
use crate::users::routes::AuthRoutes;

/// Combines all of the routes together
pub fn init(
    pool: Arc<PgPool>, // database pool
    config: Config,    // config
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    AuthRoutes::init(pool.clone(), config.clone())
        .or(ScoreboardRoutes::init(pool.clone()))
        .or(TeamRoutes::init(pool.clone()))
        .or(SearchRoutes::init(pool.clone()))
        .with(warp::log("tallii-platform"))
        .recover(handle_rejection)
}
