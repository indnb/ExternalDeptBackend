use crate::dto::request::hackathon_2025::user::User;
use crate::utils::prelude_api::*;
use crate::utils::validation;
use rocket::post;

#[utoipa::path(
    post,
    path = "/api/hackathon_2025/user/create",
    request_body = User,
    tag = "Hackathon User 2025",
    operation_id = "user_create",
    responses(
        (status = 200, description = "User created successfully"),
        (status = 422, description = "Validation error", body = ApiErrorBody),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/hackathon_2025/user/create", data = "<data>")]
pub async fn create(
    db_pool: &DbState,
    data: Json<User>,
    admin_match: AdminAuthData,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;

    let data = data.into_inner().0;

    validation::data::hackathon_2025::user::field(&data)?;

    let id = crate::diesel::utils::hackathon_2025::user::insert::new(db_pool, data)?;

    info!("Succeed create user with id {id}");

    Ok(())
}
