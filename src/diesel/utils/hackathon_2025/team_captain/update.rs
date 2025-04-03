use crate::diesel::models::hackathon_2025::team_captain::HackathonTeamCaptain2025Insertable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_team_captain_2025::dsl::*;

pub fn by_data(
    db_pool: &DbState,
    data: HackathonTeamCaptain2025Insertable,
) -> Result<usize, ApiError> {
    diesel::update(hackathon_team_captain_2025)
        .filter(team_id.eq(data.team_id))
        .set((
            captain_id.eq(data.captain_id),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut get_connection(db_pool)?)
        .map(|data| {
            if data == 0 {
                Err(ApiError::FailedToUpdateTeamCaptainByData(
                    "Captain not found".to_string(),
                ))
            } else {
                Ok(data)
            }
        })
        .map_err(|err| ApiError::FailedToUpdateTeamCaptainByData(err.to_string()))
        .and_then(|data| data)
}
