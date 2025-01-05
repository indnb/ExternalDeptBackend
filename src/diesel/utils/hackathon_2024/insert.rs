use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Insertable;
use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Insertable;
use crate::error::api_error::ApiError;
use diesel::query_dsl::RunQueryDsl;
use rocket::State;

pub fn university(
    db_pool: &State<DbPool>,
    data: HackathonUniversity2024Insertable,
) -> Result<i32, ApiError> {
    let mut db_connection = get_connection(db_pool)?;
    diesel::insert_into(crate::diesel::schema::hackathon_university_2024::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_university_2024::id)
        .get_result::<i32>(&mut db_connection)
        .map_err(|err| {
            log::error!(
                "Error inserting hackathon 2024 university with id - {:?}",
                err
            );
            ApiError::DatabaseErrorResult(err)
        })
}
pub fn team(db_pool: &State<DbPool>, data: HackathonTeam2024Insertable) -> Result<i32, ApiError> {
    let mut db_connection = get_connection(db_pool)?;
    diesel::insert_into(crate::diesel::schema::hackathon_team_2024::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_team_2024::id)
        .get_result::<i32>(&mut db_connection)
        .map_err(|err| {
            log::error!("Error inserting hackathon 2024 team with id - {:?}", err);
            ApiError::DatabaseErrorResult(err)
        })
}
