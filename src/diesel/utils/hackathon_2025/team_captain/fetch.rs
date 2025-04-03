use crate::diesel::models::hackathon_2025::team_captain::HackathonTeamCaptain2025Queryable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_team_captain_2025::dsl::*;

pub fn by_team_id(
    connection: &mut DbPooled,
    id: i32,
) -> Result<HackathonTeamCaptain2025Queryable, ApiError> {
    hackathon_team_captain_2025
        .filter(team_id.eq(id))
        .first::<HackathonTeamCaptain2025Queryable>(connection)
        .map_err(|err| ApiError::FailedToGetUserById(err.to_string()))
}

pub fn by_captain_id(
    db_pool: &DbPool,
    user_id: i32,
) -> Result<HackathonTeamCaptain2025Queryable, ApiError> {
    hackathon_team_captain_2025
        .filter(captain_id.eq(user_id))
        .first::<HackathonTeamCaptain2025Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| ApiError::FailedToGetUserById(err.to_string()))
}

pub fn all(pool: &mut DbPooled) -> Result<Vec<HackathonTeamCaptain2025Queryable>, ApiError> {
    hackathon_team_captain_2025
        .load::<HackathonTeamCaptain2025Queryable>(pool)
        .map_err(|err| ApiError::FailedToGetAllUsers(err.to_string()))
}
