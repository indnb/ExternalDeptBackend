use crate::middleware::admin_match::AdminMatch;
use crate::utils::prelude_api::*;
use rocket::delete;

#[delete("/hackathon_2024/university/by_id/<id>")]
pub async fn by_id(db_pool: &DbState, id: i32, admin_match: AdminMatch) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let id = crate::diesel::utils::hackathon_2024::university::delete::by_id(db_pool, id)?;
    info!("Successfully deleted university from hackathon_university_2024 with id {id}");
    Ok(())
}
