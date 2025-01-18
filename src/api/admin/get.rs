use crate::{middleware::admin_token_match::AdminAuthData, utils::prelude_api::*};
use rocket::get;

#[get("/admin/get")]
pub async fn get(admin: AdminAuthData) {
    info!("Auth data admin: {:?}", admin);
}
