use crate::error::api_error::ApiError;
use crate::utils::env_configuration::EnvConfiguration;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[allow(dead_code)]
pub fn send_letter(
    title_letter: String,
    body_letter: String,
    email_user: String,
) -> Result<(), ApiError> {
    let creds = Credentials::new(
        EnvConfiguration::get().smtp_email.to_owned(),
        EnvConfiguration::get().smtp_password.to_owned(),
    );

    let smtp = SmtpTransport::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .port(587)
        .build();

    let from_email = EnvConfiguration::get()
        .smtp_email
        .parse()
        .map_err(|_| ApiError::SendEmailError("Invalid sender email address".to_string()))?;

    let to_email = email_user
        .parse()
        .map_err(|_| ApiError::SendEmailError("Invalid recipient email address".to_string()))?;

    let email = Message::builder()
        .from(from_email)
        .to(to_email)
        .subject(title_letter)
        .body(body_letter)
        .map_err(|_| ApiError::SendEmailError("Failed to build email message".to_string()))?;

    match smtp.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            Ok(())
        }
        Err(e) => Err(ApiError::SendEmailError(format!("Send email error: {}", e))),
    }
}
