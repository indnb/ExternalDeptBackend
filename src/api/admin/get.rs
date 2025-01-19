use crate::{middleware::admin_token_match::AdminAuthData, utils::prelude_api::*};
use rocket::get;

#[utoipa::path(
    get,
    path = "/api/admin/get",
    operation_id = "get_admin",
    tag = "Admin",
    responses(
        (status = 200, description = "Successfully authenticated admin"),
        (status = 401, description = "Unauthorized error"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/admin/get")]
pub async fn get(admin: AdminAuthData) {
    info!("Auth data admin: {:?}", admin);
}
