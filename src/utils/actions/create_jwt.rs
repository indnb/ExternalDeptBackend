use crate::error::api_error::ApiError;
use crate::utils::env_configuration::EnvConfiguration;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}
#[allow(dead_code)]
pub fn create_jwt(data_for_jwt: String) -> Result<String, ApiError> {
    let my_claims = Claims {
        sub: data_for_jwt,
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
