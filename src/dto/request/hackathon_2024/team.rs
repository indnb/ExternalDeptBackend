use crate::diesel::models::hackathon_2024::category::HackathonCategory2024Enum;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TeamRegistrationData {
    pub id: i32,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct TeamUpdateData {
    pub id: i32,
    pub name: String,
    pub category: HackathonCategory2024Enum,
    pub nickname_tg: String,
}
