use crate::data::hackathon_2024::user::UserJwt;
use crate::diesel::models::hackathon_2024::hackathon_user_2024::HackathonUser2024Insertable;
use crate::error::api_error::ApiError;
use crate::utils::security::encoded_data;
use crate::utils::validation::data;
use rocket::serde::json::Json;
use rocket::{info, post};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
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
		"testfar07@gmail.com".to_owned(),
		"cmkvjfeqkrpqfyfz".to_owned(),
	);

	let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
		.unwrap()
		.credentials(creds)
		.port(587)
		.build();

        send_email("test".to_string(), mailer);

    info!("Email has been send with token - {}", token);

    Ok(format!("Email has been send with token - {}", token).to_string())
}
fn send_email(html_body: String, smtp: SmtpTransport) {
	let email = Message::builder()
		.from("artemk2504@gmail.com".parse().unwrap())
		.to("artemk2504@gmail.com".parse().unwrap())
		.subject("Test Email")
		.body(html_body)
		.unwrap();

	match smtp.send(&email) {
		Ok(_) => println!("Email sent successfully!"),
		Err(e) => panic!("Could not send email: {e:?}"),
	}
}