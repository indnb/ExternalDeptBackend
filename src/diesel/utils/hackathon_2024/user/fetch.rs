use crate::diesel::models::hackathon_2024::user::HackathonUser2024Queryable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_user_2024::dsl::hackathon_user_2024;
use crate::diesel::schema::hackathon_user_2024::{id, team_id, university_id};

pub fn all(db_pool: &DbState) -> Result<Vec<HackathonUser2024Queryable>, ApiError> {
    hackathon_user_2024
        .load::<HackathonUser2024Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| err.into())
}

pub fn by_id(db_pool: &DbState, path_id: i32) -> Result<HackathonUser2024Queryable, ApiError> {
    hackathon_user_2024
        .filter(id.eq(path_id))
        .first::<HackathonUser2024Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| err.into())
}

pub fn by_university(
    db_pool: &DbState,
    path_id: i32,
) -> Result<Vec<HackathonUser2024Queryable>, ApiError> {
    hackathon_user_2024
        .filter(university_id.eq(path_id))
        .load::<HackathonUser2024Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| err.into())
}

pub fn by_team(
    db_pool: &DbState,
    path_id: i32,
) -> Result<Vec<HackathonUser2024Queryable>, ApiError> {
    hackathon_user_2024
        .filter(team_id.eq(path_id))
        .load::<HackathonUser2024Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| err.into())
}
