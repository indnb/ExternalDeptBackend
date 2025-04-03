use crate::utils::prelude_api::*;
use rocket::post;

#[utoipa::path(
    delete,
    path = "/api/hackathon_2025/team_captain/by_team_id/{team_id}",
    tag = "Hackathon Team Captain 2025",
    operation_id = "delete_team_captain",
    params(
        ("team_id" = i32, Path, description = "ID of the team to get")
    ),
    responses(
        (status = 200, description = "Team captain got successfully"),
        (status = 422, description = "Validation error", body = ApiErrorBody),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
)]
#[post("/hackathon_2025/team_captain/by_team_id/<team_id>")]
pub async fn by_team_id(
    pool: &DbState,
    team_id: i32,
    admin_match: AdminAuthData,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;

    let id = crate::diesel::utils::hackathon_2025::team_captain::delete::by_team_id(pool, team_id)?;

    info!(
        "Succeed insert new hackathon 2025 team captain with team id, {:?}",
        id
    );

    Ok(())
}
