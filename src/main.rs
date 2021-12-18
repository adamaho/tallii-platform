use redis::Client;

use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub mod config;
pub mod db;
pub mod email;
pub mod errors;
pub mod handlers;
pub mod routes;
pub mod token;

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

    // init redis client
    let client = Client::open(config.redis_url.clone()).expect("Failed to connect to redis.");

    // init the routes
    let routes = routes::init(Arc::new(pool), client.clone(), config.clone());

    // start the warp server
    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;

    Ok(())
}
