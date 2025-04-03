use crate::dto::request::hackathon_2025::team::TeamUpdateData;
use crate::utils::prelude_api::*;
use rocket::put;

#[utoipa::path(
    put,
    path = "/api/hackathon_2025/team/by_data",
    tag = "Hackathon Team 2025",
    request_body = TeamUpdateData,
    operation_id = "put_team_by_data",
    responses(
        (status = 200, description = "Team updated successfully"),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/hackathon_2025/team/by_data", data = "<data>")]
pub async fn by_data(
    db_pool: &DbState,
    data: Json<TeamUpdateData>,
    admin_match: AdminAuthData,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let data = data.into_inner();
    let _ = crate::diesel::utils::hackathon_2025::team::update::by_data(db_pool, &data)?;
    info!("Successfully updating team from hackathon_team_2025 with data: {data:?}");
    Ok(())
}
