use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Queryable;
use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Queryable;
use crate::diesel::schema::hackathon_team_2024::dsl::hackathon_team_2024;
use crate::diesel::schema::hackathon_team_2024::{id as team_id, password_registration};
use crate::diesel::schema::hackathon_university_2024::dsl::hackathon_university_2024;
use crate::diesel::schema::hackathon_university_2024::id as university_id;
use crate::error::api_error::ApiError;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::State;
pub fn team(
    db_pool: &State<DbPool>,
    id: i32,
    password: impl AsRef<str>,
) -> Result<HackathonTeam2024Queryable, ApiError> {
    let mut db_connection = get_connection(db_pool)?;

    hackathon_team_2024
        .filter(team_id.eq(id))
        .filter(password_registration.eq(password.as_ref()))
        .first::<HackathonTeam2024Queryable>(&mut db_connection)
        .map_err(ApiError::DatabaseErrorResult)
}
pub fn university(
    db_pool: &State<DbPool>,
    id: i32,
) -> Result<HackathonUniversity2024Queryable, ApiError> {
    let mut db_connection = get_connection(db_pool)?;

    hackathon_university_2024
        .filter(university_id.eq(id))
        .first::<HackathonUniversity2024Queryable>(&mut db_connection)
        .map_err(ApiError::DatabaseErrorResult)
}
