use crate::dto::request::hackathon_2024::user::RegistrationData;
use crate::models::hackathon_2024::user::UserJwt;
use crate::utils::actions;
use crate::utils::prelude_api::*;
use crate::utils::security::{encoded_data, verify_password};
use crate::utils::validation;
use rocket::post;

#[post("/hackathon_2024/user/try_registration", data = "<registration_data>")]
pub async fn try_registration(
    db_pool: &DbState,
    registration_data: Json<RegistrationData>,
) -> Result<(), ApiError> {
    let registration_data = registration_data.into_inner();

    let RegistrationData {
        user_data,
        team_data,
        ..
    } = registration_data;

    validation::data::hackathon_2024::team::check_team_password(team_data.password.as_str())?;
    validation::data::hackathon_2024::user::field(&user_data)?;

    let team = crate::diesel::utils::hackathon_2024::team::fetch::by_id(db_pool, team_data.id)?;
    verify_password(team_data.password, team.password_registration.as_str())?;

    let _ = crate::diesel::utils::hackathon_2024::university::fetch::by_id(
        db_pool,
        user_data.university_id,
    )?;

    let jwt_user = UserJwt::from(&user_data, 5);

    let token = encoded_data(&jwt_user)?;

    actions::send_letter::send_letter(
        "HACKATHON".to_owned(),
        format!(
            "<html>
            <body>
                <p>Hello!</p>
                <a href=\"{}\">Accept</a>
            </body>
         </html>",
            token
        )
        .to_string(),
        user_data.nickname_tg.to_owned(),
    )?;

    info!(
        "verify email sent to {}, jwt token - {token}",
        user_data.nickname_tg
    );
    Ok(())
}
