use crate::error::api_error::ApiError;
use crate::utils::env_configuration::CONFIG;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn hashing_data(value: &mut String) -> Result<String, ApiError> {
    hash(value, DEFAULT_COST).map_err(|err| {
        log::error!("Error hashing password: {:?}", err);
        ApiError::HashingError(err.to_string())
    })
}
pub fn verify_password(password: &str, hash: &str) -> Result<bool, ApiError> {
    verify(password, hash).map_err(|err| {
        log::error!("Error verifying password: {:?}", err);
        ApiError::HashingError(err.to_string())
    })
}
pub fn decoded_data<T: DeserializeOwned>(jwt_token: &String) -> Result<TokenData<T>, ApiError> {
    decode::<T>(
        &jwt_token,
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
