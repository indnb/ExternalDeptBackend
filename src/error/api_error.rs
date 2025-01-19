use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::Request;
use serde::Serialize;
use std::io::Cursor;
use thiserror::Error;

#[derive(Error, Debug, utoipa::ToSchema)]
pub enum ApiError {
    #[allow(dead_code)]
    #[error("Error not found")]
    NotFound,

    #[allow(dead_code)]
    #[error("Database result error occurred")]
    DatabaseErrorResult(String),

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

    #[allow(dead_code)]
    #[error("Email send error")]
    SendEmailError(String),

    #[allow(dead_code)]
    #[error("Header mismatched error")]
    HeaderMismatched(String),
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        log::error!("API error occurred: {:?}", self);

        let (status, message) = match self {
            ApiError::NotFound => (Status::NotFound, "Resource not found".to_owned()),

            ApiError::DatabaseErrorResult(err) => (
                Status::InternalServerError,
                format!("Database error result: {}", err),
            ),

            ApiError::InternalServerError => (
                Status::InternalServerError,
                "Internal server error".to_owned(),
            ),

            ApiError::BadRequest => (Status::BadRequest, "Bad request".to_owned()),

            ApiError::HttpError => (Status::BadGateway, "HTTP error".to_owned()),

            ApiError::HashingError(err) => (Status::InternalServerError, err),

            ApiError::ValidationError(err) => (Status::UnprocessableEntity, err),

            ApiError::TokenGenerationError(err) => (Status::InternalServerError, err),

            ApiError::TokenDecodeError(err) => (Status::Unauthorized, err),

            ApiError::Unauthorized(err) => (Status::Unauthorized, err),

            ApiError::InvalidClaims => (Status::Unauthorized, "Invalid claims".to_owned()),

            ApiError::DatabaseErrorConnection(err) => (Status::ServiceUnavailable, err),

            ApiError::SendEmailError(err) => (Status::InternalServerError, err),

            ApiError::HeaderMismatched(err) => (Status::BadRequest, err),
        };

        let body = serde_json::to_string(&ApiErrorBody {
            error: status.to_string(),
            message,
        })
        .unwrap_or("Can't deserialize api error body".to_owned());

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

impl From<diesel::result::Error> for ApiError {
    fn from(err: diesel::result::Error) -> Self {
        ApiError::DatabaseErrorResult(err.to_string())
    }
}
