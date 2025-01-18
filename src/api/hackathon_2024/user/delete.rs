use crate::utils::prelude_api::*;
use rocket::delete;

#[delete("/hackathon_2024/user/by_id/<user_id>")]
pub async fn by_id(
    db_pool: &DbState,
    user_id: i32,
    admin_match: AdminAuthData,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let user_id = crate::diesel::utils::hackathon_2024::user::delete::by_id(db_pool, user_id)?;
    info!("Successfully deleted hackathon_user_2024 from hackathon 2024 with id {user_id}");
    Ok(())
}
