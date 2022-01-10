use std::sync::Arc;

use jsonwebtoken::TokenData;
use sqlx::PgPool;
use warp::Filter;

use super::handlers;
use super::token::Claims;
use crate::config::Config;
use crate::wrappers::{with_auth, with_config, with_pool};

pub struct AuthRoutes;

impl AuthRoutes {
    /// Init the auth routes
    pub fn init(
        pool: Arc<PgPool>,
        config: Config,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let auth_routes = authorize().or(login(pool.clone())
            .or(signup(pool.clone(), config.clone()))
            .or(get_me(pool.clone()))
            .or(update_me(pool.clone())));

        auth_routes
    }
}

/// GET /v1/authorize - Validates a token
pub fn authorize() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "authorize")
        .and(warp::get())
        .and(with_auth())
        .map(move |_: TokenData<Claims>| warp::reply())
}

/// GET /v1/me - gets the currently logged in users profile
pub fn get_me(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "me")
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::get_me)
}

/// PUT /v1/me - updates the currently logged in users profile
pub fn update_me(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "me")
        .and(warp::put())
        .and(warp::body::json())
        .and(with_pool(pool.clone()))
        .and(with_auth())
        .and_then(handlers::update_me)
}

/// Logs a user into the applicaton
pub fn login(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(pool.clone()))
        .and_then(handlers::login)
}

/// Signs a user up
pub fn signup(
    pool: Arc<PgPool>,
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "signup")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(pool.clone()))
        .and(with_config(config.clone()))
        .and_then(handlers::signup)
}
