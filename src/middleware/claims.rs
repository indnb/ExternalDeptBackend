use crate::utils::env_configuration::CONFIG;
use chrono::{Duration, Utc};
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

impl Claims {
    /* WILL UNCOMMENT WHEN IN SCHEMA.RS EXISTS USER_ROLE!!!
    #[allow(dead_code)]
    pub async fn check_admin(db_pool: &State<DbPool>, claims: Claims) -> Result<(), ApiError> {
        if get_role(db_pool, claims)
            .await
            .map_err(|_| ApiError::DatabaseError(diesel::NotFound))?
            .into_inner()
            != CONFIG.get().unwrap().admin_role
        {
            return Err(ApiError::Unauthorized("Claims not authorized".to_string()));
        }
        Ok(())
    }
    */
}

impl Claims {
    #[allow(dead_code)]
    pub fn new(sub: i32) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        Claims {
            exp: expiration,
            sub,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    #[allow(dead_code)]
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
                    Err(e) => {
                        warn!("Error decoding token: {:?}", e);
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
