use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_team_2024::dsl::hackathon_team_2024;
use crate::diesel::schema::hackathon_team_2024::{category, name, nickname_tg, updated_at};
use crate::dto::request::hackathon_2024::team::TeamUpdateData;

pub fn by_data(db_pool: &DbState, data: &TeamUpdateData) -> Result<usize, ApiError> {
    diesel::update(hackathon_team_2024)
        .filter(crate::diesel::schema::hackathon_team_2024::id.eq(data.id))
        .set((
            name.eq(&data.name),
            category.eq(data.category),
            nickname_tg.eq(&data.nickname_tg),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut get_connection(db_pool)?)
        .map(|data| {
            if data == 0 {
                Err(ApiError::FailedToUpdateTeamByData(
                    "Team not found".to_string(),
                ))
            } else {
                Ok(data)
            }
        })
        .map_err(|err| ApiError::FailedToUpdateTeamByData(err.to_string()))
        .and_then(|data| data)
}
