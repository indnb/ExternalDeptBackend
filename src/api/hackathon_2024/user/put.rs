use crate::dto::request::hackathon_2024::user::User;
use crate::utils::prelude_api::*;
use rocket::put;

#[utoipa::path(
    put,
    path = "/api/hackathon_2024/user/by_id/{id}",
    request_body = User,
    tag = "Hackathon User 2024",
    operation_id = "update_user_by_id",
    params(
        ("id" = i32, Path, description = "ID of the user to update")
    ),
    responses(
        (status = 200, description = "User updated successfully", body = String),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/hackathon_2024/user/by_id/<id>", data = "<data>")]
pub async fn by_id(
    db_pool: &DbState,
    data: Json<User>,
    id: i32,
    admin_match: AdminAuthData,
) -> Result<String, ApiError> {
    admin_match.check_admin()?;
    let data = data.into_inner();
    crate::diesel::utils::hackathon_2024::user::update::by_id(db_pool, id, &data.0)?;
    Ok(format!(
        "Successfully updated hackathon_user_2024 with email: {}",
        data.0.nickname_tg
    ))
}
