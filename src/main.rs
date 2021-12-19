use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub mod auth;
pub mod config;
pub mod errors;
pub mod routes;
pub mod wrappers;

use crate::config::Config;
use crate::errors::TalliiError;

type ResponseResult<T> = std::result::Result<T, warp::Rejection>;
type Result<T> = std::result::Result<T, TalliiError>;

#[tokio::main]
async fn main() -> Result<()> {
    // get config from the env
    let config = Config::from_env();

    pretty_env_logger::init();

    // configure the databse pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database.");

    // init the routes
    let routes = routes::init(Arc::new(pool), config.clone());

    // start the warp server
    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;

    Ok(())
}
