use crate::dto::response::hackathon_2025::team::{Team, VecTeam};
use crate::utils::prelude_api::*;
use rocket::get;

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/team/all",
    tag = "Hackathon Team 2024",
    operation_id = "get_all_team",
    responses(
        (status = 200, description = "All team get successfully", body = Vec<Team>),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
)]
#[get("/hackathon_2025/team/all")]
pub async fn all(db_pool: &DbState) -> Result<Json<VecTeam>, ApiError> {
    Ok(Json(VecTeam(
        crate::diesel::utils::hackathon_2025::team::fetch::all(db_pool)?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/team/by_id/{id}",
    tag = "Hackathon Team 2024",
    operation_id = "get_team_by_id",
    params(
        ("id" = i32, Path, description = "ID of the team to get")
    ),
    responses(
        (status = 200, description = "Team get successfully", body = Team),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
)]
#[get("/hackathon_2025/team/by_id/<id>")]
pub async fn by_id(db_pool: &DbState, id: i32) -> Result<Json<Team>, ApiError> {
    Ok(Json(Team(
        crate::diesel::utils::hackathon_2025::team::fetch::by_id(db_pool, id)?,
    )))
}
