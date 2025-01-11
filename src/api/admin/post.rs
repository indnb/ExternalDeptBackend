use crate::dto::request::admin::admin::LoginAdminData;
use crate::error::api_error::ApiError;
use crate::utils::admin::create_jwt;
use crate::utils::env_configuration::EnvConfiguration;
use crate::utils::prelude_api::*;
use rocket::post;

#[post("/admin/admin_login", data = "<data>")]
pub async fn admin_login(data: Json<LoginAdminData>) -> Result<String, ApiError> {
    let data = data.into_inner();
    let LoginAdminData { admin_password } = data;
    let password_env = EnvConfiguration::get().admin_password.to_owned();
    if password_env != admin_password {
        return Err(ApiError::ValidationError("passow".to_string()));
    }
    match create_jwt::create_jwt(admin_password) {
        Ok(token) => Ok(format!("{}", token)),
        Err(err) => {
            println!("Ошибка при создании токена: {:?}", err); // Печатаем ошибку
            Err(ApiError::TokenGenerationError(err.to_string())) // Возвращаем ошибку с нужным типом
        }
    }
}
