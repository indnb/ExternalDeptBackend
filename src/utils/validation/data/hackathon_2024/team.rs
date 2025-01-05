use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Insertable;
use crate::error::api_error::ApiError;
use crate::utils::validation::validation_string::Validate;

#[allow(dead_code)]
pub fn field(new_team: &HackathonTeam2024Insertable) -> Result<(), ApiError> {
    check_email(new_team.email)?;
    check_name(new_team.name)?;
    check_password(new_team.password_registration)?;
    Ok(())
}
pub fn check_password(password: &str) -> Result<(), ApiError> {
    let max_lenght = 20;
    if !password.is_password(max_lenght) {
        Ok(())
    } else {
        Err(ApiError::ValidationError(
            format!(
                "Team password greater for {} symbol or don't correct regex",
                max_lenght
            )
            .to_owned(),
        ))
    }
}

fn check_name(name: &str) -> Result<(), ApiError> {
    let max_lenght = 30;
    if !name.less_for(max_lenght) {
        Ok(())
    } else {
        Err(ApiError::ValidationError(
            format!("Team name greater for {} symbol", max_lenght).to_owned(),
        ))
    }
}
fn check_email(email: &str) -> Result<(), ApiError> {
    if !email.is_email() {
        Ok(())
    } else {
        Err(ApiError::ValidationError(
            format!("Email don't correct {}", email).to_owned(),
        ))
    }
}
