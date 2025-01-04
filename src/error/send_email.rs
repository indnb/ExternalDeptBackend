use crate::utils::env_configuration::EnvConfiguration;
use lettre::{Message, SmtpTransport, Transport};
pub fn send_email(html_body: String, smtp: SmtpTransport, email_user: String) {
    let email = Message::builder()
        .from(EnvConfiguration::get().smtp_email.parse().unwrap())
        .to(email_user.parse().unwrap())
        .subject("Test Email")
        .body(html_body)
        .unwrap();

    match smtp.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
