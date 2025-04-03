use crate::{
    diesel::{
        models::hackathon_2025::team_captain::HackathonTeamCaptain2025Queryable,
        prelude::get_connection,
    },
    dto::response::hackathon_2025::team_captain::{TeamCaptainResponse, VecTeamCaptainResponse},
    utils::prelude_api::*,
};
use rocket::get;

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/team_captain/by_team_id/{team_id}",
    tag = "Hackathon Team Captain 2025",
    operation_id = "fetch_team_captain_by_team_id",
    params(
        ("team_id" = i32, Path, description = "ID of the team to get")
    ),
    responses(
        (status = 200, description = "Team captain got successfully", body = TeamCaptainResponse),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2025/team_captain/by_team_id/<team_id>")]
pub async fn by_team_id(
    pool: &DbState,
    team_id: i32,
    admin_match: AdminAuthData,
) -> Result<Json<TeamCaptainResponse>, ApiError> {
    admin_match.check_admin()?;

    Ok(Json(TeamCaptainResponse(
        crate::diesel::utils::hackathon_2025::team_captain::fetch::by_team_id(
            &mut get_connection(pool)?,
            team_id,
        )?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/team_captain/by_captain_id/{captain_id}",
    tag = "Hackathon Team Captain 2025",
    operation_id = "fatch_team_captain_by_captain_id",
    params(
        ("captain_id" = i32, Path, description = "ID of the captain to get")
    ),
    responses(
        (status = 200, description = "Team captain got successfully", body = TeamCaptainResponse),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2025/team_captain/by_captain_id/<team_id>")]
pub async fn by_captain_id(
    pool: &DbState,
    team_id: i32,
    admin_match: AdminAuthData,
) -> Result<Json<TeamCaptainResponse>, ApiError> {
    admin_match.check_admin()?;

    Ok(Json(TeamCaptainResponse(
        crate::diesel::utils::hackathon_2025::team_captain::fetch::by_captain_id(pool, team_id)?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/team_captain/all",
    tag = "Hackathon Team Captain 2025",
    operation_id = "fetch_team_captain_all",
    responses(
        (status = 200, description = "Team captains got successfully", body = Vec<HackathonTeamCaptain2025Queryable>),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2025/team_captain/all")]
pub async fn all(
    pool: &DbState,
    admin_match: AdminAuthData,
) -> Result<Json<VecTeamCaptainResponse>, ApiError> {
    admin_match.check_admin()?;

    Ok(Json(VecTeamCaptainResponse(
        crate::diesel::utils::hackathon_2025::team_captain::fetch::all(&mut get_connection(pool)?)?,
    )))
}
