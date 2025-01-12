use crate::models::admin::admin_jwt;
use crate::utils::actions::decode_jwt;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, warn, Request};

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
            Some(token) => match decode_jwt::decode_jwt(token.to_string()) {
                Ok(token_data) => request::Outcome::Success(token_data),
                Err(e) => {
                    warn!("Error decoding token: {:?}", e);
                    request::Outcome::Error((Status::Unauthorized, ()))
                }
            },
            None => {
                warn!("Token not found in header \"Authorization\"");
                request::Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}
