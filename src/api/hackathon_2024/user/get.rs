use crate::api::hackathon_2024::user::local::create_user_by_jwt;
use crate::dto::response::hackathon_2024::user::User;
use crate::dto::response::hackathon_2024::user::VecUser;
use crate::middleware::admin_match::AdminMatch;
use crate::middleware::claims::Claims;
use crate::utils::prelude_api::*;
use crate::utils::security::decoded_data;
use rocket::get;

#[get("/hackathon_2024/user/confirm_new_user?<jwt_token>")]
pub async fn confirm_new_user(db_pool: &DbState, jwt_token: String) -> Result<(), ApiError> {
    create_user_by_jwt(db_pool, decoded_data(&jwt_token)?.claims)
}

#[allow(dead_code)]
#[get("/hackathon_2024/user/authorization_user")]
pub async fn authorization_user(db_pool: &DbState, claims: Claims) -> Result<Json<User>, ApiError> {
    Ok(Json(User(
        crate::diesel::utils::hackathon_2024::user::fetch::by_id(db_pool, claims.sub)?,
    )))
}

#[get("/hackathon_2024/user/all")]
pub async fn all(db_pool: &DbState, admin_match: AdminMatch) -> Result<Json<VecUser>, ApiError> {
    admin_match.check_admin()?;
    Ok(Json(VecUser(
        crate::diesel::utils::hackathon_2024::user::fetch::all(db_pool)?,
    )))
}
