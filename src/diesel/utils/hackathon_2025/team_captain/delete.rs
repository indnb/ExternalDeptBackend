use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_team_captain_2025::dsl::*;

pub fn by_team_id(pool: &DbState, id: i32) -> Result<usize, ApiError> {
    diesel::delete(hackathon_team_captain_2025)
        .filter(team_id.eq(id))
        .execute(&mut get_connection(pool)?)
        .map(|data| {
            if data == 0 {
                Err(ApiError::FailedToDeleteTeamCaptainByTeamId(
                    "Team not found".to_string(),
                ))
            } else {
                Ok(data)
            }
        })
        .map_err(|err| ApiError::FailedToDeleteTeamCaptainByTeamId(err.to_string()))
        .and_then(|data| data)
}
