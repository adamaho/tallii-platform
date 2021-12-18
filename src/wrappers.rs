use std::sync::Arc;

use jsonwebtoken::TokenData;
use sqlx::PgPool;
use warp::Filter;

use crate::auth::token::Claims;
use crate::config::Config;

use crate::errors::TalliiError;
use crate::ResponseResult;

/// Extracts jwt from request in the Authorization header and verfies it
pub fn with_auth() -> impl Filter<Extract = (TokenData<Claims>,), Error = warp::Rejection> + Clone {
    warp::header::headers_cloned().and_then(validate_jwt)
}

/// Extracts claims from request in the Authorization header
pub fn with_claims() -> impl Filter<Extract = (TokenData<Claims>,), Error = warp::Rejection> + Clone
{
    warp::header::headers_cloned().and_then(decode_jwt)
}

/// Extracts database pool
pub fn with_pool(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = (Arc<PgPool>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

/// Extracts config
pub fn with_config(
    config: Config,
) -> impl Filter<Extract = (Config,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}

/// Validates the jwt token
async fn decode_jwt(
    headers: warp::http::HeaderMap<warp::http::HeaderValue>,
) -> ResponseResult<TokenData<Claims>> {
    match jwt_from_headers(&headers) {
        Ok(token) => Claims::decode_jwt(token.to_string()).map_err(|e| warp::reject::custom(e)),
        Err(_) => Err(warp::reject::custom(TalliiError::MissingBearerToken)),
    }
}

/// Validates the jwt token
async fn validate_jwt(
    headers: warp::http::HeaderMap<warp::http::HeaderValue>,
) -> ResponseResult<TokenData<Claims>> {
    match jwt_from_headers(&headers) {
        Ok(token) => Claims::verify_jwt(token.to_string()).map_err(|e| warp::reject::custom(e)),
        Err(_) => Err(warp::reject::custom(TalliiError::MissingBearerToken)),
    }
}

/// Get the jwt token from the headers
fn jwt_from_headers(headers: &warp::http::HeaderMap<warp::http::HeaderValue>) -> Result<&str, ()> {
    // get the authorization header
    if let Some(authorization) = headers.get("Authorization") {
        // convert the auth header to a string
        let auth_header = authorization.to_str().unwrap();

        // if the auth header does not start with Bearer, return an error
        if !&auth_header.starts_with("Bearer") {
            return Err(());
        }

        // strip the bearer from the header to get the token
        if let Some(token) = auth_header.strip_prefix("Bearer") {
            Ok(token.trim())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
