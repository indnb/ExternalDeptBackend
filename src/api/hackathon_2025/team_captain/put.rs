use crate::dto::request::hackathon_2025::team_captain::TeamCaptain;
use crate::utils::prelude_api::*;
use rocket::put;

#[utoipa::path(
    put,
    path = "/api/hackathon_2025/team_captain/by_data",
    request_body = TeamCaptain,
    tag = "Hackathon Team Captain 2025",
    operation_id = "update_team_captain",
    responses(
        (status = 200, description = "Team captain updated successfully"),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/hackathon_2025/team_captain/by_data", data = "<data>")]
pub async fn by_data(
    pool: &DbState,
    data: Json<TeamCaptain>,
    admin_match: AdminAuthData,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;

    let data = data.into_inner();
    let id =
        crate::diesel::utils::hackathon_2025::team_captain::update::by_data(pool, data.into())?;

    info!(
        "Succeed insert new hackathon 2025 team captain with team id, {}",
        id
    );

    Ok(())
}
