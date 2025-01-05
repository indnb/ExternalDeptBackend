use serde::Deserialize;
use crate::diesel::models::hackathon_2024::user::HackathonUser2024Insertable;
use crate::data::hackathon_2024::team::TeamRegistrationData;

#[derive(Deserialize)]
pub struct RegistrationData {
    pub user_data: HackathonUser2024Insertable,
    pub team_data: TeamRegistrationData,
}
