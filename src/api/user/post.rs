use crate::data::user::UserJwt;
use crate::diesel::models::users_data::users::UserInsertable;
use crate::error::api_error::ApiError;
use crate::utils::env_configuration::CONFIG;
use crate::utils::validation;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rocket::serde::json::Json;
use rocket::{info, post};

#[post("/user/try_registration", data = "<user_data>")]
pub async fn try_registration(user_data: Json<UserInsertable<'_>>) -> Result<String, ApiError> {
    let mut new_user = user_data.into_inner();

    let _ = validation::data::user::field(&mut new_user)?;

    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(5))
        .expect("Failed to compute expiration time")
        .timestamp() as usize;

    let jwt_user = UserJwt {
        first_name: new_user.first_name.to_string(),
        last_name: new_user.last_name.to_string(),
        password: new_user.password.to_string(),
        email: new_user.email.to_string(),
        phone: new_user.phone.to_string(),
        role: new_user.role,
        exp,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &jwt_user,
        &EncodingKey::from_secret(CONFIG.get().unwrap().jwt_secret.as_ref()),
    )
    .map_err(|_| ApiError::InternalServerError)?;

    info!("Token generated: {}", token);

    Ok(format!("Email has been send with token - {}", token).to_string())
}
