use crate::dto::request::hackathon_2024::team::TeamUpdateData;
use crate::utils::prelude_api::*;
use rocket::put;

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
