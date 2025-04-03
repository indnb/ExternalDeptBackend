use crate::dto::request::hackathon_2025::team_captain::TeamCaptain;
use crate::utils::prelude_api::*;
use rocket::post;

#[utoipa::path(
    post,
    path = "/api/hackathon_2025/team_captain/by_data",
    request_body = TeamCaptain,
    tag = "Hackathon Team Captain 2025",
    operation_id = "create_team_captain",
    responses(
        (status = 200, description = "Team captain created successfully"),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/hackathon_2025/team_captain/by_data", data = "<data>")]
pub async fn by_data(
    pool: &DbState,
    data: Json<TeamCaptain>,
    admin_match: AdminAuthData,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;

    let data = data.into_inner();
    let id = crate::diesel::utils::hackathon_2025::team_captain::insert::new(pool, data.into())?;

    info!(
        "Succeed insert new hackathon 2025 team captain with team id, {}",
        id
    );

    Ok(())
}
