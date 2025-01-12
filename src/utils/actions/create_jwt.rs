use crate::error::api_error::ApiError;
use crate::models::admin::admin_jwt;
use crate::utils::env_configuration::EnvConfiguration;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

#[allow(dead_code)]
pub fn create_jwt(admin_password: String, admin_name: String) -> Result<String, ApiError> {
    let my_claims = admin_jwt::AdminJwt {
        admin_password,
        admin_name,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as u64,
    };
    let secret_key = EnvConfiguration::get().jwt_secret.to_owned();
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .map_err(|err| ApiError::TokenGenerationError(err.to_string()))?;

    Ok(token.to_string())
}
