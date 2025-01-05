use crate::diesel::database_diesel::DbPool;
use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Insertable;
use crate::error::api_error::ApiError;
use rocket::serde::json::Json;
use rocket::{post, State};

#[post("/hackathon_2024/team/create", data = "<data>")]
pub async fn create(
    db_pool: &State<DbPool>,
    data: Json<HackathonTeam2024Insertable<'_>>,
) -> Result<String, ApiError> {
    let data = data.into_inner();
    crate::utils::validation::data::hackathon_2024::team::field(&data)?;
    let id = crate::diesel::utils::hackathon_2024::insert::team(db_pool, data)?;
    Ok(format!("Succeed insert new hackathon 2024 team with id - {}", id).to_owned())
}
