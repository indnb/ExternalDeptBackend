use crate::error::api_error::ApiError;
use crate::utils::env_configuration::CONFIG;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, warn, Request};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminMatch {
    is_admin: bool,
}

impl AdminMatch {
    #[allow(dead_code)]
    pub fn new(is_admin: bool) -> Self {
        AdminMatch { is_admin }
    }
    #[allow(dead_code)]
    pub fn check_admin(&self) -> Result<bool, ApiError> {
        match &self.is_admin {
            true => Ok(true),
            false => Err(ApiError::Unauthorized(
                "Don't admin matching password".to_owned(),
            )),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminMatch {
    type Error = ();

    #[allow(dead_code)]
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let header_password = req.headers().get_one("Admin");

        match header_password {
            Some(header_password) => {
                let env_password = CONFIG.get().unwrap().admin_password.as_str();
                if header_password == env_password {
                    request::Outcome::Success(AdminMatch { is_admin: true })
                } else {
                    warn!("Error admin password");
                    request::Outcome::Error((Status::Unauthorized, ()))
                }
            }
            None => {
                warn!("Token not found in header \"Authorization\"");
                request::Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}
