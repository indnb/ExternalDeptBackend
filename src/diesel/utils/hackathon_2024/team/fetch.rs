use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Queryable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_team_2024::dsl::hackathon_team_2024;
pub fn by_id(db_pool: &DbState, team_id: i32) -> Result<HackathonTeam2024Queryable, ApiError> {
    hackathon_team_2024
        .filter(crate::diesel::schema::hackathon_team_2024::columns::id.eq(team_id))
        .first::<HackathonTeam2024Queryable>(&mut get_connection(db_pool)?)
        .map_err(ApiError::DatabaseErrorResult)
}
pub fn all(db_pool: &State<DbPool>) -> Result<Vec<HackathonTeam2024Queryable>, ApiError> {
    hackathon_team_2024
        .load::<HackathonTeam2024Queryable>(&mut get_connection(db_pool)?)
        .map_err(ApiError::DatabaseErrorResult)
}
