use crate::diesel::models::hackathon_2024::user::HackathonUser2024Insertable;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserJwt {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub team_id: i32,
    pub university: i32,
    pub exp: usize,
}
impl UserJwt {
    pub fn from(data: &HackathonUser2024Insertable, exp: i64) -> Self {
        let exp = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(exp))
            .expect("Failed to compute expiration time")
            .timestamp() as usize;
        Self {
            first_name: data.first_name.to_owned(),
            last_name: data.last_name.to_owned(),
            email: data.email.to_owned(),
            phone: data.phone.to_owned(),
            team_id: data.team_id,
            university: data.university,
            exp,
        }
    }
}
