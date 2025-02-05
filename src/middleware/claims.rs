use crate::utils::env_configuration::CONFIG;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, warn, Request};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "));

        match token {
            Some(token) => {
                let secret = CONFIG.get().unwrap().jwt_secret.as_str();

                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::new(Algorithm::HS512),
                ) {
                    Ok(token_data) => request::Outcome::Success(token_data.claims),
                    Err(err) => {
                        warn!("Error decoding token: {err:?}");
                        request::Outcome::Error((Status::Unauthorized, ()))
                    }
                }
            }
            None => {
                warn!("Token not found in header \"Authorization\"");
                request::Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}
