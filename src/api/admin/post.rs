use crate::dto::request::admin::login_admin::LoginAdminData;
use crate::error::api_error::ApiError;
use crate::utils::actions;
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
    match actions::create_jwt::create_jwt(admin_password) {
        Ok(token) => Ok(token),
        Err(err) => {
            println!("Ошибка при создании токена: {:?}", err); 
            Err(ApiError::TokenGenerationError(err.to_string())) 
        }
    }
}
