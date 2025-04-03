use crate::diesel::models::hackathon_2025::team::HackathonTeam2025Queryable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_team_2025::dsl::hackathon_team_2025;

pub fn by_id(db_pool: &DbState, team_id: i32) -> Result<HackathonTeam2025Queryable, ApiError> {
    hackathon_team_2025
        .filter(crate::diesel::schema::hackathon_team_2025::columns::id.eq(team_id))
        .first::<HackathonTeam2025Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| ApiError::FailedToGetTeamById(err.to_string()))
}

pub fn all(db_pool: &State<DbPool>) -> Result<Vec<HackathonTeam2025Queryable>, ApiError> {
    hackathon_team_2025
        .load::<HackathonTeam2025Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| ApiError::FailedToGetAllTeams(err.to_string()))
}
