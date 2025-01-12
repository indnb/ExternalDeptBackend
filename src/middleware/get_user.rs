use crate::models::admin::admin_jwt;
use crate::utils::security;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for admin_jwt::AdminJwt {
    type Error = ();

    #[allow(dead_code)]
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "));

        match token {
            Some(token) => match security::decoded_data::<admin_jwt::AdminJwt>(token) {
                Ok(token_data) => request::Outcome::Success(token_data.claims),
                Err(_) => request::Outcome::Error((Status::Unauthorized, ())),
            },
            None => request::Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
