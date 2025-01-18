use crate::utils::prelude_api::*;
use rocket::delete;

#[delete("/hackathon_2024/team/by_id/<id>")]
pub async fn by_id(db_pool: &DbState, id: i32, admin_match: AdminAuthData) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let id = crate::diesel::utils::hackathon_2024::team::delete::by_id(db_pool, id)?;
    info!("Successfully deleted team from hackathon_team_2024 with id {id}");
    Ok(())
}
