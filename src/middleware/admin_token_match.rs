use crate::diesel::prelude::ApiError;
use crate::utils::prelude_api::EnvConfiguration;
use crate::utils::security;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct AdminAuthData {
    pub admin_password: String,
    pub admin_name: String,
    pub exp: u64,
}

impl AdminAuthData {
    pub fn check_admin(&self) -> Result<(), ApiError> {
        log::info!(
            "Admin name: {}, Admin password: {}",
            EnvConfiguration::get().admin_name,
            EnvConfiguration::get().admin_password
        );
        if self.admin_name != EnvConfiguration::get().admin_name {
            Err(ApiError::InvalidAdminName(format!(
                "Error validation admin name: {}",
                self.admin_name
            )))
        } else if self.admin_password != EnvConfiguration::get().admin_password {
            Err(ApiError::InvalidAdminPassword(format!(
                "Error validation admin password: {}",
                self.admin_password
            )))
        } else {
            Ok(())
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminAuthData {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "));

        match token {
            Some(token) => match security::decoded_data::<AdminAuthData>(token) {
                Ok(token_data) => request::Outcome::Success(token_data.claims),
                Err(_) => request::Outcome::Error((
                    Status::Unauthorized,
                    ApiError::AdminUnauthorized("Admin not authorized from token".to_string()),
                )),
            },
            None => request::Outcome::Error((
                Status::Unauthorized,
                ApiError::AdminHeaderMismatch("Admin header mismatched".to_owned()),
            )),
        }
    }
}
