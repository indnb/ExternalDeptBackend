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
    #[error("Database error occurred")]
    DatabaseError(#[from] diesel::result::Error),
    #[allow(dead_code)]
    #[error("Internal server error")]
    InternalServerError,
    #[allow(dead_code)]
    #[error("Bad request error")]
    BadRequest,
    #[allow(dead_code)]
    #[error("HTTP error")]
    HttpError,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        log::error!("API error occurred: {:?}", self);

        let (status, message) = match self {
            ApiError::NotFound => (Status::NotFound, "Error not found"),
            ApiError::DatabaseError(_) => (Status::InternalServerError, "Database error occurred"),
            ApiError::InternalServerError => (Status::InternalServerError, "Internal server error"),
            ApiError::BadRequest => (Status::BadRequest, "Bad request error"),
            ApiError::HttpError => (Status::InternalServerError, "HTTP error occurred"),
        };

        let body = serde_json::to_string(&ApiErrorBody {
            error: status.to_string(),
            message: message.to_string(),
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
