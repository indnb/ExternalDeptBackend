use crate::dto::request::hackathon_2024::university::University;
use crate::utils::prelude_api::*;
use rocket::post;

#[utoipa::path(
    post,
    path = "/api/hackathon_2024/university/create",
    request_body = University, tag = "Hackathon University 2024",
    operation_id = "create_university",
    responses(
        (status = 200, description = "University created successfully"),
        (status = 401, description = "Unauthorized error", body = ApiErrorBody),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/hackathon_2024/university/create", data = "<data>")]
pub async fn create(
    db_pool: &DbState,
    data: Json<University>,
    admin_match: AdminAuthData,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let id = crate::diesel::utils::hackathon_2024::university::insert::new(
        db_pool,
        data.into_inner().0,
    )?;
    info!("Succeed insert new university with id - {id}");
    Ok(())
}
