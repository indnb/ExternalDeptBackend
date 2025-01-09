use crate::diesel::models::hackathon_2024::user::HackathonUser2024Insertable;
use crate::dto::request::hackathon_2024::team::TeamRegistrationData;
use serde::Deserialize;

#[non_exhaustive]
#[derive(Deserialize)]
pub struct RegistrationData {
    pub user_data: HackathonUser2024Insertable,
    pub team_data: TeamRegistrationData,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct User(pub HackathonUser2024Insertable);

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct VecUser(pub Vec<HackathonUser2024Insertable>);
