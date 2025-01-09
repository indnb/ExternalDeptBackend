use crate::dto::request::hackathon_2024::team::TeamCreateData;
use crate::utils::prelude_api::*;
use crate::utils::security::hashing_data;
use rocket::post;

#[post("/hackathon_2024/team/create", data = "<data>")]
pub async fn create(db_pool: &DbState, data: Json<TeamCreateData>) -> Result<(), ApiError> {
    let mut team = data.into_inner().0;

    crate::utils::validation::data::hackathon_2024::team::field(&team)?;
    team.password_registration = hashing_data(team.password_registration)?;

    let id = crate::diesel::utils::hackathon_2024::team::insert::new(db_pool, team)?;

    info!("Succeed insert new hackathon 2024 team with id - {id}");

    Ok(())
}
