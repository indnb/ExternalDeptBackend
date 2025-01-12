use crate::dto::request::admin::login_admin::LoginAdminData;
use crate::error::api_error::ApiError;
use crate::models::admin::admin_jwt;
use crate::utils::env_configuration::EnvConfiguration;
use crate::utils::prelude_api::*;
use crate::utils::security;
use chrono::{Duration, Utc};
use rocket::post;

#[post("/admin/login", data = "<data>")]
pub async fn login(data: Json<LoginAdminData>) -> Result<String, ApiError> {
    let data = data.into_inner();
    let LoginAdminData {
        admin_name,
        admin_password,
    } = data;
    let password_env = EnvConfiguration::get().admin_password.to_owned();
    let name_env = EnvConfiguration::get().admin_name.to_owned();
    if name_env != admin_name {
        return Err(ApiError::ValidationError(
            "Error validation admin name".to_string(),
        ));
    }

    if password_env != admin_password {
        return Err(ApiError::ValidationError(
            "Error validation admin password".to_string(),
        ));
    }
    let my_claims = admin_jwt::AdminJwt {
        admin_password,
        admin_name,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as u64,
    };

    match security::encoded_data(&my_claims) {
        Ok(token) => Ok(token),
        Err(err) => Err(ApiError::TokenGenerationError(err.to_string())),
    }
}
