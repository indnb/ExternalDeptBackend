use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Insertable;
use crate::error::api_error::ApiError;
use crate::utils::validation::data::fields::{check_name, check_nickname_tg, check_password};

#[allow(dead_code)]
pub fn field(new_team: &HackathonTeam2024Insertable) -> Result<(), ApiError> {
    check_nickname_tg(
        &new_team.nickname_tg,
        format!("Email don't correct {}", new_team.nickname_tg).as_str(),
    )?;

    check_name(
        &new_team.name,
        30,
        format!("Team name greater for {} symbol", 30).as_str(),
    )?;

    check_team_password(&new_team.password_registration)?;

    Ok(())
}

pub fn check_team_password(password: impl AsRef<str>) -> Result<(), ApiError> {
    check_password(
        password.as_ref(),
        20,
        format!(
            "Team password greater for {} symbol or don't correct regex",
            20
        )
        .as_str(),
    )?;

    Ok(())
}
