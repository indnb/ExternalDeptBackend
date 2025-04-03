use diesel::PgConnection;

use crate::diesel::models::hackathon_2025::team::HackathonTeam2025Insertable;
use crate::diesel::prelude::*;

pub fn new_tx(tx: &mut PgConnection, data: HackathonTeam2025Insertable) -> Result<i32, ApiError> {
    diesel::insert_into(crate::diesel::schema::hackathon_team_2025::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_team_2025::id)
        .get_result::<i32>(tx)
        .map_err(|err| ApiError::FailedToInsertTeam(err.to_string()))
}

pub fn new(pool: &DbState, data: HackathonTeam2025Insertable) -> Result<i32, ApiError> {
    diesel::insert_into(crate::diesel::schema::hackathon_team_2025::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_team_2025::id)
        .get_result::<i32>(&mut get_connection(pool)?)
        .map_err(|err| ApiError::FailedToInsertTeam(err.to_string()))
}
