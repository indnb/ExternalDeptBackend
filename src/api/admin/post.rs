use crate::dto::request::admin::login_admin::LoginAdminData;
use crate::error::api_error::ApiError;
use crate::utils::actions;
use crate::utils::env_configuration::EnvConfiguration;
use crate::utils::prelude_api::*;
use rocket::post;

#[post("/admin/admin_login", data = "<data>")]
pub async fn admin_login(data: Json<LoginAdminData>) -> Result<String, ApiError> {
    let data = data.into_inner();
    let LoginAdminData {
        admin_name,
        admin_password,
    } = data;
    let password_env = EnvConfiguration::get().admin_password.to_owned();
    let name_env = EnvConfiguration::get().admin_name.to_owned();
    if name_env != admin_name {
        return Err(ApiError::ValidationError("name".to_string()));
    }

    if password_env != admin_password {
        return Err(ApiError::ValidationError("passow".to_string()));
    }
    match actions::create_jwt::create_jwt(admin_password, admin_name) {
        Ok(token) => Ok(token),
        Err(err) => {
            println!("Ошибка при создании токена: {:?}", err);
            Err(ApiError::TokenGenerationError(err.to_string()))
        }
    }
}
