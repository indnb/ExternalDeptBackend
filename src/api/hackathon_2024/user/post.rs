use crate::data::hackathon_2024::user::UserJwt;
use crate::diesel::models::hackathon_2024::hackathon_user_2024::HackathonUser2024Insertable;
use crate::error::api_error::ApiError;
use crate::utils::actions;
use crate::utils::env_configuration::EnvConfiguration;
use crate::utils::security::encoded_data;
use crate::utils::validation::data;
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use rocket::post;
use rocket::serde::json::Json;
#[post("/hackathon_2024/user/try_registration", data = "<user_data>")]
pub async fn try_registration(
    user_data: Json<HackathonUser2024Insertable<'_>>,
) -> Result<String, ApiError> {
    let mut new_user = user_data.into_inner();

    data::hackathon_2024::user::field(&mut new_user)?;

    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(5))
        .expect("Failed to compute expiration time")
        .timestamp() as usize;

    let jwt_user = UserJwt {
        first_name: new_user.first_name.to_string(),
        last_name: new_user.last_name.to_string(),
        email: new_user.email.to_string(),
        phone: new_user.phone.to_string(),
        category: new_user.category,
        university: new_user.university,
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
        new_user.email.to_owned(),
    ) {
        Ok(_) => Ok(format!("verify email sent to {}", new_user.email)),
        Err(e) => Err(e),
    }
}
