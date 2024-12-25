use crate::utils::env_configuration::CONFIG;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, warn, Request};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminMatch {
    pub is_admin: bool,
}

impl AdminMatch {
    #[allow(dead_code)]
    pub fn new(is_admin: bool) -> Self {
        AdminMatch { is_admin }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminMatch {
    type Error = ();

    #[allow(dead_code)]
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = req.headers().get_one("Admin");

        match token {
            Some(token) => {
                let secret = CONFIG.get().unwrap().jwt_secret.as_str();
                if token == secret {
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
