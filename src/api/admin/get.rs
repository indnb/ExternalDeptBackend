use crate::models::admin::admin_jwt;
use rocket::get;
#[allow(dead_code)]
#[get("/admin/get")]
pub async fn admin_get(claims: admin_jwt::AdminJwt) {
    println!("Welcome, user with ID: {}", claims.admin_name);
}
