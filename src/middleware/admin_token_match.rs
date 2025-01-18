use crate::diesel::prelude::ApiError;
use crate::utils::prelude_api::EnvConfiguration;
use crate::utils::security;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminAuthData {
    pub admin_password: String,
    pub admin_name: String,
    pub exp: u64,
}

impl AdminAuthData {
    pub fn check_admin(&self) -> Result<(), ApiError> {
        if self.admin_name != EnvConfiguration::get().admin_name {
            Err(ApiError::ValidationError(
                "Error validation admin name".to_string(),
            ))
        } else if self.admin_password != EnvConfiguration::get().admin_password {
            Err(ApiError::ValidationError(
                "Error validation admin password".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminAuthData {
    type Error = ApiError;

    #[allow(dead_code)]
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
                    ApiError::Unauthorized("Admin not authorized".to_string()),
                )),
            },
            None => request::Outcome::Error((
                Status::Unauthorized,
                ApiError::HeaderMismatched("Admin header mismatched".to_owned()),
            )),
        }
    }
}
