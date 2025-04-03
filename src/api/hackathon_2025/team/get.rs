use crate::diesel::prelude::get_connection;
use crate::dto::response::hackathon_2025::team::{FullTeam, Team, VecTeam};
use crate::utils::prelude_api::*;
use rocket::get;

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/team/all",
    tag = "Hackathon Team 2025",
    operation_id = "get_all_team",
    responses(
        (status = 200, description = "All team get successfully", body = Vec<Team>),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
)]
#[get("/hackathon_2025/team/all")]
pub async fn all(pool: &DbState) -> Result<Json<VecTeam>, ApiError> {
    Ok(Json(VecTeam(
        crate::diesel::utils::hackathon_2025::team::fetch::all(&mut get_connection(pool)?)?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/team/by_id/{id}",
    tag = "Hackathon Team 2025",
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
pub async fn by_id(pool: &DbState, id: i32) -> Result<Json<Team>, ApiError> {
    Ok(Json(Team(
        crate::diesel::utils::hackathon_2025::team::fetch::by_id(&mut get_connection(pool)?, id)?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/team/by_id_full/{id}",
    tag = "Hackathon Team 2025",
    operation_id = "get_team_by_id_full",
    params(
        ("id" = i32, Path, description = "ID of the team to get")
    ),
    responses(
        (status = 200, description = "Team get successfully", body = FullTeam),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2025/team/by_id_full/<id>")]
pub async fn by_id_full(
    pool: &DbState,
    id: i32,
    admin_match: AdminAuthData,
) -> Result<Json<FullTeam>, ApiError> {
    admin_match.check_admin()?;

    Ok(Json(
        crate::diesel::utils::hackathon_2025::team::fetch::by_id_full(
            &mut get_connection(pool)?,
            id,
        )?,
    ))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/team/all_full",
    tag = "Hackathon Team 2025",
    operation_id = "get_team_all_full",
    responses(
        (status = 200, description = "Team get successfully", body = Vec<FullTeam>),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2025/team/all_full")]
pub async fn all_full(
    pool: &DbState,
    admin_match: AdminAuthData,
) -> Result<Json<Vec<FullTeam>>, ApiError> {
    admin_match.check_admin()?;

    Ok(Json(
        crate::diesel::utils::hackathon_2025::team::fetch::all_full(&mut get_connection(pool)?)?,
    ))
}
