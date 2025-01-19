use crate::utils::prelude_api::*;
use rocket::delete;

#[utoipa::path(
    delete,
    path = "/api/hackathon_2024/team/by_id/{id}",
    tag = "Hackathon Team 2024",
    operation_id = "delete_team_by_id",
    params(
        ("id" = i32, Path, description = "ID of the team to delete")
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
#[delete("/hackathon_2024/team/by_id/<id>")]
pub async fn by_id(db_pool: &DbState, id: i32, admin_match: AdminAuthData) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let id = crate::diesel::utils::hackathon_2024::team::delete::by_id(db_pool, id)?;
    info!("Successfully deleted team from hackathon_team_2024 with id {id}");
    Ok(())
}
