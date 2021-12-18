use std::convert::Infallible;

use serde::Serialize;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

/// Response to the client when an error occurs
#[derive(Serialize)]
pub struct ErrorResponse {
    code: String,
    message: String,
}

/// Representation of all potential application errors
#[derive(Error, Debug)]
pub enum TalliiError {
    #[error("Failed to execute database query")]
    DatabaseError(String),

    #[error("Couldn't reach mars. Check back later.")]
    InternalServerError(String),

    #[error("Invalid credentials")]
    Unauthorized,

    #[error("Missing Bearer token")]
    MissingBearerToken,

    #[error("The provided token is invalid")]
    InvalidToken,

    #[error("Validation error: {0}")]
    ValidationError(String),
}

/// Required in order for warp to accept the TalliiError as a valid rejection
impl warp::reject::Reject for TalliiError {}

/// Handles all application related errors and returns a common error payload.
pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let status_code;
    let code: String;
    let message: String;

    if err.is_not_found() {
        status_code = StatusCode::NOT_FOUND;
        message = String::from("Not Found");
        code = String::from("NOT_FOUND");
    } else if let Some(error) = err.find::<warp::filters::body::BodyDeserializeError>() {
        status_code = StatusCode::BAD_REQUEST;
        message = format!("{:?}", error);
        code = String::from("INVALID_REQUEST_BODY");
    } else if let Some(e) = err.find::<TalliiError>() {
        match e {
            TalliiError::DatabaseError(error) => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                message = format!("{:?}", error);
                code = String::from("DATABASE_ERROR");
            }
            TalliiError::InternalServerError(error) => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                message = format!("{:?}", error);
                code = String::from("INTERNAL_SERVER_ERROR");
            }
            TalliiError::ValidationError(error) => {
                status_code = StatusCode::BAD_REQUEST;
                message = format!("{:?}", error);
                code = String::from("VALIDATION_ERROR");
            }
            TalliiError::Unauthorized => {
                status_code = StatusCode::UNAUTHORIZED;
                message = "The provided credentials are invalid.".to_string();
                code = String::from("UNAUTHORIZED");
            }
            TalliiError::MissingBearerToken => {
                status_code = StatusCode::UNAUTHORIZED;
                message = "Missing Bearer token.".to_string();
                code = String::from("UNAUTHORIZED");
            }
            TalliiError::InvalidToken => {
                status_code = StatusCode::UNAUTHORIZED;
                message = "The provided token is invalid.".to_string();
                code = String::from("UNAUTHORIZED");
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        status_code = StatusCode::METHOD_NOT_ALLOWED;
        message = String::from("Method Not Allowed");
        code = String::from("INTERNAL_SERVER_ERROR");
    } else {
        status_code = StatusCode::INTERNAL_SERVER_ERROR;
        message = String::from("Internal Server Error");
        code = String::from("INTERNAL_SERVER_ERROR");
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.to_string(),
        code: code.to_string(),
    });

    Ok(warp::reply::with_status(json, status_code))
}
