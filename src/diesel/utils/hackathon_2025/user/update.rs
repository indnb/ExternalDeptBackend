use crate::diesel::models::hackathon_2025::user::HackathonUser2025Insertable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_user_2025::dsl::hackathon_user_2025;
use crate::diesel::schema::hackathon_user_2025::{
    first_name, id, last_name, nickname_tg, phone, team_id, university_id, updated_at,
};

pub fn by_id(
    dp_pool: &DbState,
    user_id: i32,
    data: &HackathonUser2025Insertable,
) -> Result<usize, ApiError> {
    diesel::update(hackathon_user_2025.filter(id.eq(user_id)))
        .set((
            first_name.eq(&data.first_name),
            last_name.eq(&data.last_name),
            nickname_tg.eq(&data.nickname_tg),
            phone.eq(&data.phone),
            university_id.eq(data.university_id),
            team_id.eq(data.team_id),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut get_connection(dp_pool)?)
        .map(|data| {
            if data == 0 {
                Err(ApiError::FailedToUpdateUserById(
                    "User not found".to_string(),
                ))
            } else {
                Ok(data)
            }
        })
        .map_err(|err| ApiError::FailedToUpdateUserById(err.to_string()))
        .and_then(|data| data)
}
