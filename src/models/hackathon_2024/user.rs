use crate::diesel::models::hackathon_2024::user::HackathonUser2024Insertable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJwt {
    pub first_name: String,
    pub last_name: String,
    pub nickname_tg: String,
    pub phone: String,
    pub team_id: i32,
    pub university_id: i32,
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
            nickname_tg: data.nickname_tg.to_owned(),
            phone: data.phone.to_owned(),
            team_id: data.team_id,
            university_id: data.university_id,
            exp,
        }
    }
    pub fn into(self) -> HackathonUser2024Insertable {
        HackathonUser2024Insertable {
            first_name: self.first_name,
            last_name: self.last_name,
            nickname_tg: self.nickname_tg,
            phone: self.phone,
            team_id: self.team_id,
            university_id: self.university_id,
        }
    }
}
