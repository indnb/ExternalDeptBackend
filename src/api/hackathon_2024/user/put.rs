use crate::dto::request::hackathon_2024::user::User;
use crate::middleware::admin_match::AdminMatch;
use crate::utils::prelude_api::*;
use rocket::put;
#[put("/hackathon_2024/user/by_id/<user_id>", data = "<data>")]
pub async fn by_id(
    db_pool: &DbState,
    data: Json<User>,
    user_id: i32,
    admin_match: AdminMatch,
) -> Result<String, ApiError> {
    admin_match.check_admin()?;
    let data = data.into_inner();
    crate::diesel::utils::hackathon_2024::user::update::by_id(db_pool, user_id, &data.0)?;
    Ok(format!(
        "Successfully updated hackathon_user_2024 with email: {}",
        data.0.nickname_tg
    ))
}
