use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_team_2025::dsl::hackathon_team_2025;
use crate::diesel::schema::hackathon_team_2025::{category, name, updated_at};
use crate::dto::request::hackathon_2025::team::UpdateTeam;

pub fn by_data(db_pool: &DbState, data: &UpdateTeam) -> Result<usize, ApiError> {
    diesel::update(hackathon_team_2025)
        .filter(crate::diesel::schema::hackathon_team_2025::id.eq(data.id))
        .set((
            name.eq(&data.name),
            category.eq(data.category),
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
