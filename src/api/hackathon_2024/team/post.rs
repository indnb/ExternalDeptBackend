use crate::diesel::database_diesel::DbPool;
use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Insertable;
use crate::error::api_error::ApiError;
use crate::utils::security::hashing_data;
use rocket::serde::json::Json;
use rocket::{post, State};

#[post("/hackathon_2024/team/create", data = "<data>")]
pub async fn create(
    db_pool: &State<DbPool>,
    data: Json<HackathonTeam2024Insertable<'_>>,
) -> Result<String, ApiError> {
    let mut team = data.into_inner();

    crate::utils::validation::data::hackathon_2024::team::field(&team)?;

    let hashed_password = hashing_data(team.password_registration)?;
    team.password_registration = hashed_password.as_str();

    let id = crate::diesel::utils::hackathon_2024::insert::team(db_pool, team)?;
    Ok(format!("Succeed insert new hackathon 2024 team with id - {}", id).to_owned())
}
