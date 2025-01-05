use crate::data::admin_match::AdminMatch;
use crate::diesel::database_diesel::DbPool;
use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Insertable as University;
use crate::error::api_error::ApiError;
use rocket::serde::json::Json;
use rocket::{post, State};

#[post("/hackathon_2024/university/create", data = "<data>")]
pub async fn create(
    db_pool: &State<DbPool>,
    data: Json<University<'_>>,
    admin_match: AdminMatch,
) -> Result<String, ApiError> {
    admin_match.check_admin()?;
    let data = data.into_inner();
    let id = crate::diesel::utils::hackathon_2024::insert::university(db_pool, data)?;
    Ok(format!("Succeed insert new university with id - {}", id).to_owned())
}
