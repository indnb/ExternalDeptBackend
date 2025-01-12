use crate::models::admin::admin_jwt;
use log::info;
use rocket::get;

#[allow(dead_code)]
#[get("/admin/get")]
pub async fn get(claims: admin_jwt::AdminJwt) {
    info!("Welcome, user with ID: {}", claims.admin_name);
}
