use crate::diesel::models::hackathon_2024::hackathon_category::HackathonCategory2024Enum;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserJwt {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub category: HackathonCategory2024Enum,
    pub university: i32,
    pub exp: usize,
}
