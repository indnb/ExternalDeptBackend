use crate::data::hackathon_2024::api::RegistrationData;
use crate::data::hackathon_2024::user::UserJwt;
use crate::diesel::database_diesel::DbPool;
use crate::error::api_error::ApiError;
use crate::utils::actions;
use crate::utils::security::{encoded_data, verify_password};
use crate::utils::validation;
use rocket::serde::json::Json;
use rocket::{post, State};

#[post("/hackathon_2024/user/try_registration", data = "<registration_data>")]
pub async fn try_registration(
    db_pool: &State<DbPool>,
    registration_data: Json<RegistrationData>,
) -> Result<String, ApiError> {
    let registration_data = registration_data.into_inner();

    let RegistrationData {
        user_data,
        team_data,
        ..
    } = registration_data;

    validation::data::hackathon_2024::team::check_team_password(team_data.password.as_str())?;
    validation::data::hackathon_2024::user::field(&user_data)?;

    let team = crate::diesel::utils::hackathon_2024::fetch::team(db_pool, team_data.id)?;
    verify_password(team_data.password, team.password_registration.as_str())?;

    let _ = crate::diesel::utils::hackathon_2024::fetch::university(db_pool, user_data.university)?;

    let jwt_user = UserJwt::from(&user_data, 5);

    let token = encoded_data(&jwt_user)?;

    actions::send_letter::send_letter(
        "letter".to_owned(),
        format!(
            "<html>
            <body>
                <p>Здравствуйте!</p>
                <p>Чтобы подтвердить участие в хакатоне, пожалуйста, перейдите по ссылке:</p>
                <a href=\"{}\">Подтвердить участие</a>
            </body>
         </html>",
            token
        )
        .to_string(),
        user_data.email.to_owned(),
    )?;

    Ok(format!(
        "verify email sent to {}, jwt token - {}",
        user_data.email, token
    ))
}
