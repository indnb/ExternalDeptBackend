use crate::error::api_error::ApiError;
use crate::utils::env_configuration::EnvConfiguration;
use lettre::{Message, SmtpTransport, Transport};
pub fn send_confirm_email(
    html_body: String,
    smtp: SmtpTransport,
    email_user: String,
) -> Result<(), ApiError> {
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
        .subject("Test Email")
        .body(html_body)
        .map_err(|_| ApiError::SendEmailError("Failed to build email message".to_string()))?;

    match smtp.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            Ok(())
        }
        Err(e) => Err(ApiError::SendEmailError(format!("Send email error: {}", e))),
    }
}
