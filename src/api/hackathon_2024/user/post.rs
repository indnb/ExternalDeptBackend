use crate::data::hackathon_2024::api::RegistrationData;
use crate::data::hackathon_2024::user::UserJwt;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Queryable;
use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Queryable;
use crate::diesel::schema::hackathon_team_2024::dsl::hackathon_team_2024;
use crate::diesel::schema::hackathon_team_2024::{id, password_registration};
use crate::diesel::schema::hackathon_university_2024::dsl::hackathon_university_2024;
use crate::diesel::schema::hackathon_university_2024::id as university_id;
use crate::error::api_error::ApiError;
use crate::utils::actions;
use crate::utils::env_configuration::EnvConfiguration;
use crate::utils::security::encoded_data;
use crate::utils::validation;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use rocket::serde::json::Json;
use rocket::{post, State};
#[post("/hackathon_2024/user/try_registration", data = "<registration_data>")]
pub async fn try_registration(
    db_pool: &State<DbPool>,
    registration_data: Json<RegistrationData>,
) -> Result<String, ApiError> {
    let mut connection = get_connection(db_pool)?;
    let registration_data = registration_data.into_inner();
    let RegistrationData {
        user_data,
        team_data,
        ..
    } = registration_data;
    validation::data::hackathon_2024::team::check_password(team_data.password.as_str())?;
    validation::data::hackathon_2024::user::field(&user_data)?;

    let _ = hackathon_team_2024
        .filter(id.eq(team_data.id))
        .filter(password_registration.eq(team_data.password.as_str()))
        .first::<HackathonTeam2024Queryable>(&mut connection)
        .map_err(ApiError::DatabaseErrorResult)?;

    let _ = hackathon_university_2024
        .filter(university_id.eq(user_data.university))
        .first::<HackathonUniversity2024Queryable>(&mut connection)
        .map_err(ApiError::DatabaseErrorResult)?;

    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(5))
        .expect("Failed to compute expiration time")
        .timestamp() as usize;
    let jwt_user = UserJwt {
        first_name: user_data.first_name.to_owned(),
        last_name: user_data.last_name.to_owned(),
        email: user_data.email.to_owned(),
        phone: user_data.phone.to_owned(),
        team_id: user_data.team_id,
        university: user_data.university,
        exp,
    };

    let token = encoded_data(&jwt_user)?;
    let creds = Credentials::new(
        EnvConfiguration::get().smtp_email.to_owned(),
        EnvConfiguration::get().smtp_password.to_owned(),
    );

    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .port(587)
        .build();

    match actions::send_letter::send_letter(
        "letter".to_string(),
        format!(
            "<html>
            <body>
                <p>Здравствуйте!</p>
                <p>Чтобы подтвердить участие в хакатоне, пожалуйста, перейдите по ссылке:</p>
                <a href=\"{token}\">Подтвердить участие</a>
            </body>
         </html>"
        )
        .to_string(),
        mailer,
        user_data.email.to_owned(),
    ) {
        Ok(_) => Ok(format!("verify email sent to {}", user_data.email)),
        Err(e) => Err(e),
    }
}
