use crate::diesel::models::hackathon_2024::user::HackathonUser2024Insertable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_user_2024::dsl::hackathon_user_2024;
use crate::diesel::schema::hackathon_user_2024::{
    first_name, id, last_name, nickname_tg, phone, university_id, updated_at,
};

pub fn by_id(
    dp_pool: &State<DbPool>,
    user_id: i32,
    data: &HackathonUser2024Insertable,
) -> Result<usize, ApiError> {
    diesel::update(hackathon_user_2024.filter(id.eq(user_id)))
        .set((
            first_name.eq(&data.first_name),
            last_name.eq(&data.last_name),
            nickname_tg.eq(&data.nickname_tg),
            phone.eq(&data.phone),
            university_id.eq(data.university_id),
            id.eq(data.team_id),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut get_connection(dp_pool)?)
        .map_err(|err| {
            error!("Error updating hackathon_user_2024, bellow error");
            ApiError::DatabaseErrorResult(err)
        })
}
