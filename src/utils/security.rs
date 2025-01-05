use crate::error::api_error::ApiError;
use crate::utils::env_configuration::CONFIG;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[allow(dead_code)]
pub fn hashing_data(value: impl AsRef<str>) -> Result<String, ApiError> {
    hash(value.as_ref(), DEFAULT_COST).map_err(|err| {
        log::error!("Error hashing password: {:?}", err);
        ApiError::HashingError(err.to_string())
    })
}
#[allow(dead_code)]
pub fn verify_password(password: impl AsRef<str>, hash: impl AsRef<str>) -> Result<bool, ApiError> {
    verify(password.as_ref(), hash.as_ref()).map_err(|err| {
        log::error!("Error verifying password: {:?}", err);
        ApiError::HashingError(err.to_string())
    })
}
pub fn decoded_data<T: DeserializeOwned>(
    jwt_token: impl AsRef<str>,
) -> Result<TokenData<T>, ApiError> {
    decode::<T>(
        jwt_token.as_ref(),
        &DecodingKey::from_secret(CONFIG.get().unwrap().jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|err| ApiError::TokenDecodeError(err.to_string()))
}
pub fn encoded_data<T: Serialize>(value: &T) -> Result<String, ApiError> {
    encode(
        &Header::new(Algorithm::HS512),
        &value,
        &EncodingKey::from_secret(CONFIG.get().unwrap().jwt_secret.as_ref()),
    )
    .map_err(|_| ApiError::InternalServerError)
}
