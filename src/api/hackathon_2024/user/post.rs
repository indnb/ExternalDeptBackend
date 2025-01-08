use crate::dto::request::hackathon_2024::user::RegistrationData;
use crate::utils::prelude_api::*;
use crate::utils::security::verify_password;
use crate::utils::validation;
use rocket::post;

#[post("/hackathon_2024/user/registration_by_tg", data = "<data>")]
pub async fn registration_by_tg(
    db_pool: &DbState,
    data: Json<RegistrationData>,
) -> Result<(), ApiError> {
    let data = data.into_inner();

    let RegistrationData {
        user_data,
        team_data,
    } = data;

    validation::data::hackathon_2024::team::check_team_password(team_data.password.as_str())?;
    validation::data::hackathon_2024::user::field(&user_data)?;

    let team = crate::diesel::utils::hackathon_2024::team::fetch::by_id(db_pool, team_data.id)?;
    verify_password(
        team_data.password.as_str(),
        team.password_registration.as_str(),
    )?;

    let id = crate::diesel::utils::hackathon_2024::user::insert::new(db_pool, user_data)?;

    info!("Succeed create user with id {id}");

    Ok(())
}
