use crate::error::api_error::ApiError;
use crate::models::admin::admin_jwt;
use crate::utils::env_configuration::EnvConfiguration;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub fn decode_jwt(token: String) -> Result<admin_jwt::AdminJwt, ApiError> {
    let secret_key = EnvConfiguration::get().jwt_secret.to_owned();

    match decode::<admin_jwt::AdminJwt>(
        &token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    ) {
        Ok(decoded_token) => Ok(decoded_token.claims), // Return the decoded claims (Admin_Jwt)
        Err(err) => Err(ApiError::TokenGenerationError(err.to_string())),
    }
}
