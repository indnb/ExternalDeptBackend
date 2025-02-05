use crate::dto::request::hackathon_2024::user::RegistrationData;
use crate::utils::prelude_api::*;
use crate::utils::security::verify_password;
use crate::utils::validation;
use rocket::post;

#[utoipa::path(
    post,
    path = "/api/hackathon_2024/user/registration_by_tg",
    request_body = RegistrationData,
    tag = "Hackathon User 2024",
    operation_id = "user_registration_by_tg",
    responses(
        (status = 200, description = "User registration successfully"),
        (status = 422, description = "Validation error", body = ApiErrorBody),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
)]
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
    validation::data::hackathon_2024::team::check_team_members_count(team.count_members)?;

    verify_password(
        team_data.password.as_str(),
        team.password_registration.as_str(),
    )?;

    let id = crate::diesel::utils::hackathon_2024::user::insert::new(db_pool, user_data)?;

    info!("Succeed create user with id {id}");

    Ok(())
}
