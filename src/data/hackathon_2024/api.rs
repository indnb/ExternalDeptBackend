use crate::data::hackathon_2024::team::TeamRegistrationData;
use crate::diesel::models::hackathon_2024::user::HackathonUser2024Insertable;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegistrationData {
    pub user_data: HackathonUser2024Insertable,
    pub team_data: TeamRegistrationData,
}
