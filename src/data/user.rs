use crate::diesel::models::users_data::users_role::UserRoleEnum;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserLoginRequest {
    #[allow(dead_code)]
    pub email: String,
    #[allow(dead_code)]
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub email: String,
    #[allow(dead_code)]
    pub token: String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserJwt {
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub role: UserRoleEnum,
    pub exp: usize,
}
