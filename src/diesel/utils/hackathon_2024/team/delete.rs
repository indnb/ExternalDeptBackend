use crate::diesel::configurator::get_connection;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_team_2024::dsl::hackathon_team_2024;
use crate::error::api_error::ApiError;

pub fn by_id(db_pool: &DbState, id: i32) -> Result<usize, ApiError> {
    diesel::delete(hackathon_team_2024)
        .filter(crate::diesel::schema::hackathon_team_2024::id.eq(id))
        .execute(&mut get_connection(db_pool)?)
        .map(|data| {
            if data == 0 {
                Err(ApiError::FailedToDeleteTeamById(
                    "Team not found".to_string(),
                ))
            } else {
                Ok(data)
            }
        })
        .map_err(|err| ApiError::FailedToDeleteTeamById(err.to_string()))
        .and_then(|data| data)
}
