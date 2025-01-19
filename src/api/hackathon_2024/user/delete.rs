use crate::utils::prelude_api::*;
use rocket::delete;

#[utoipa::path(
    delete,
    path = "/api/hackathon_2024/user/by_id/{user_id}",
    tag = "Hackathon User 2024",
    operation_id = "update_user_by_id",
    params(
        ("id" = i32, Path, description = "ID of the user to delete")
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[delete("/hackathon_2024/user/by_id/<user_id>")]
pub async fn by_id(
    db_pool: &DbState,
    user_id: i32,
    admin_match: AdminAuthData,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let user_id = crate::diesel::utils::hackathon_2024::user::delete::by_id(db_pool, user_id)?;
    info!("Successfully deleted hackathon_user_2024 from hackathon 2024 with id {user_id}");
    Ok(())
}
