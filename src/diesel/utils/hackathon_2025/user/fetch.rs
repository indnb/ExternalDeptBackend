use crate::diesel::models::hackathon_2025::user::HackathonUser2025Queryable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_user_2025::dsl::hackathon_user_2025;
use crate::diesel::schema::hackathon_user_2025::{id, team_id, university_id};

pub fn all(db_pool: &DbState) -> Result<Vec<HackathonUser2025Queryable>, ApiError> {
    hackathon_user_2025
        .load::<HackathonUser2025Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| ApiError::FailedToGetAllUsers(err.to_string()))
}

pub fn by_id(db_pool: &DbState, path_id: i32) -> Result<HackathonUser2025Queryable, ApiError> {
    hackathon_user_2025
        .filter(id.eq(path_id))
        .first::<HackathonUser2025Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| ApiError::FailedToGetUserById(err.to_string()))
}

pub fn by_university(
    db_pool: &DbState,
    path_id: i32,
) -> Result<Vec<HackathonUser2025Queryable>, ApiError> {
    hackathon_user_2025
        .filter(university_id.eq(path_id))
        .load::<HackathonUser2025Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| ApiError::FailedToGetUsersByUniversity(err.to_string()))
}

pub fn by_team(
    db_pool: &DbState,
    path_id: i32,
) -> Result<Vec<HackathonUser2025Queryable>, ApiError> {
    hackathon_user_2025
        .filter(team_id.eq(path_id))
        .load::<HackathonUser2025Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| ApiError::FailedToGetUsersByTeam(err.to_string()))
}
