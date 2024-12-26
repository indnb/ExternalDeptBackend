use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::Request;
use serde::Serialize;
use std::io::Cursor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[allow(dead_code)]
    #[error("Error not found")]
    NotFound,
    #[allow(dead_code)]
    #[error("Database result error occurred")]
    DatabaseErrorResult(#[from] diesel::result::Error),
    #[allow(dead_code)]
    #[error("Database connection error occurred")]
    DatabaseErrorConnection(String),
    #[allow(dead_code)]
    #[error("Internal server error")]
    InternalServerError,
    #[allow(dead_code)]
    #[error("Bad request error")]
    BadRequest,
    #[allow(dead_code)]
    #[error("HTTP error")]
    HttpError,
    #[allow(dead_code)]
    #[error("Hashing error")]
    HashingError(String),
    #[allow(dead_code)]
    #[error("Validation error")]
    ValidationError(String),
    #[allow(dead_code)]
    #[error("Token generation error")]
    TokenGenerationError(String),
    #[allow(dead_code)]
    #[error("Token decoded error")]
    TokenDecodeError(String),
    #[allow(dead_code)]
    #[error("Unauthorized error")]
    Unauthorized(String),
    #[allow(dead_code)]
    #[error("Invalid claims error")]
    InvalidClaims,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        log::error!("API error occurred: {:?}", self);

        let (status, message) = match self {
            ApiError::NotFound => (Status::NotFound, "Error not found".to_owned()),
            ApiError::DatabaseErrorResult(_) => (
                Status::InternalServerError,
                "Database error occurred".to_owned(),
            ),
            ApiError::InternalServerError => (
                Status::InternalServerError,
                "Internal server error".to_owned(),
            ),
            ApiError::BadRequest => (Status::BadRequest, "Bad request error".to_owned()),
            ApiError::HttpError => (
                Status::InternalServerError,
                "HTTP error occurred".to_owned(),
            ),
            ApiError::HashingError(_) => (Status::NotFound, "Hashing error".to_owned()),
            ApiError::ValidationError(_) => (Status::NotFound, "Validation error".to_owned()),
            ApiError::TokenGenerationError(_) => {
                (Status::NotFound, "Token generation error".to_owned())
            }
            ApiError::TokenDecodeError(_) => (Status::NotFound, "Token decoded error".to_owned()),
            ApiError::Unauthorized(err) => (
                Status::Unauthorized,
                format!("Unauthorized error - {}", err).to_owned(),
            ),
            ApiError::InvalidClaims => (Status::Unauthorized, "Invalid claims error".to_owned()),
            ApiError::DatabaseErrorConnection(_) => (
                Status::InternalServerError,
                "Database connection error".to_owned(),
            ),
        };

        let body = serde_json::to_string(&ApiErrorBody {
            error: status.to_string(),
            message,
        })
        .expect("Failed to serialize error body");

        Response::build()
            .status(status)
            .sized_body(body.len(), Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}

#[derive(Serialize)]
struct ApiErrorBody {
    error: String,
    message: String,
}
