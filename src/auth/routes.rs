use std::sync::Arc;

use jsonwebtoken::TokenData;
use sqlx::PgPool;
use warp::Filter;

use crate::auth::handlers;
use crate::auth::token::Claims;
use crate::wrappers::{with_auth, with_pool};

pub struct AuthRoutes;

impl AuthRoutes {
    /// Init the auth routes
    pub fn init(
        pool: Arc<PgPool>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let auth_routes = authorize().or(login(pool.clone()));

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

/// Takes an email and token from url and verifies it
pub fn login(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(pool.clone()))
        .and_then(handlers::login)
}
