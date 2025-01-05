use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Insertable;
use crate::error::api_error::ApiError;
use crate::utils::validation::data::fields::{check_email, check_name, check_password};

#[allow(dead_code)]
pub fn field(new_team: &HackathonTeam2024Insertable) -> Result<(), ApiError> {
    check_email(
        new_team.email,
        format!("Email don't correct {}", new_team.email).as_str(),
    )?;
    check_name(
        new_team.name,
        30,
        format!("Team name greater for {} symbol", 30).as_str(),
    )?;
    check_team_password(new_team.password_registration)?;
    Ok(())
}

pub fn check_team_password(password: &str) -> Result<(), ApiError> {
    check_password(
        password,
        20,
        format!(
            "Team password greater for {} symbol or don't correct regex",
            20
        ).as_str(),
    )?;
    Ok(())
}
