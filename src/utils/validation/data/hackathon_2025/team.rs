use crate::dto::request::hackathon_2025::team::NewTeam;
use crate::error::api_error::ApiError;
use crate::utils::validation::data::fields::{check_name, check_password};

pub async fn field(team: &NewTeam) -> Result<(), ApiError> {
    check_name(
        &team.name,
        30,
        format!("Team name greater for {} symbol", 30).as_str(),
    )?;

    check_team_members_count(team.hackathon_users.len())?;

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

pub fn check_team_members_count(count: usize) -> Result<(), ApiError> {
    if count >= 6 {
        Err(ApiError::InvalidTeamMembersCount(
            "Team members count greater for 6".to_string(),
        ))
    } else {
        Ok(())
    }
}
