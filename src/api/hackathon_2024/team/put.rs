use crate::dto::request::hackathon_2024::team::TeamUpdateData;
use crate::utils::prelude_api::*;
use rocket::put;

#[utoipa::path(
    put,
    path = "/api/hackathon_2024/team/by_id",
    tag = "Hackathon Team 2024",
    request_body = TeamUpdateData,
    operation_id = "put_team_by_id",
    params(
        ("id" = i32, Path, description = "ID of the team to update")
    ),
    responses(
        (status = 200, description = "Team updated successfully"),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/hackathon_2024/team/by_id", data = "<data>")]
pub async fn by_id(
    db_pool: &DbState,
    data: Json<TeamUpdateData>,
    admin_match: AdminAuthData,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let data = data.into_inner();
    let _ = crate::diesel::utils::hackathon_2024::team::update::by_data(db_pool, &data)?;
    info!("Successfully updating team from hackathon_team_2024 with data: {data:?}");
    Ok(())
}
