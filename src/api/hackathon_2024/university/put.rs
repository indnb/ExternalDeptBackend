use crate::dto::request::hackathon_2024::university::University;
use crate::utils::prelude_api::*;
use rocket::put;

#[put("/hackathon_2024/university/by_id/<id>", data = "<data>")]
pub async fn by_id(
    db_pool: &DbState,
    id: i32,
    admin_match: AdminAuthData,
    data: Json<University>,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let id = crate::diesel::utils::hackathon_2024::university::update::by_id(
        db_pool,
        id,
        data.into_inner().0,
    )?;
    info!("Successfully updated hackathon_university_2024 with id: {id}");
    Ok(())
}
