use crate::dto::request::hackathon_2024::university::University;
use crate::middleware::admin_match::AdminMatch;
use crate::utils::prelude_api::*;
use rocket::post;

#[post("/hackathon_2024/university/create", data = "<data>")]
pub async fn create(
    db_pool: &DbState,
    data: Json<University>,
    admin_match: AdminMatch,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let id = crate::diesel::utils::hackathon_2024::university::insert::new(
        db_pool,
        data.into_inner().0,
    )?;
    info!("Succeed insert new university with id - {id}");
    Ok(())
}
