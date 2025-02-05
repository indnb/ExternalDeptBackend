use crate::dto::request::admin::login_admin::LoginAdminData;
use crate::error::api_error::ApiError;
use crate::middleware::admin_token_match::AdminAuthData;
use crate::utils::prelude_api::*;
use crate::utils::security;
use chrono::{Duration, Utc};
use rocket::post;

#[utoipa::path(
    post,
    path = "/api/admin/login",
    request_body = LoginAdminData,
    operation_id = "login_admin",
    tag = "Admin",
    responses(
        (status = 200, description = "Login successfully", body = String),
        (status = 422, description = "Validation error", body = ApiErrorBody),
    ),
)]
#[post("/admin/login", data = "<data>")]
pub async fn login(data: Json<LoginAdminData>) -> Result<String, ApiError> {
    let data = data.into_inner();
    let LoginAdminData {
        admin_name,
        admin_password,
    } = data;

    let auth = AdminAuthData {
        admin_password,
        admin_name,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as u64,
    };

    auth.check_admin()?;

    match security::encoded_data(&auth) {
        Ok(token) => Ok(token),
        Err(err) => Err(ApiError::FailedToGenerateAdminToken(err.to_string())),
    }
}
