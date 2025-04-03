use crate::diesel::{
    models::hackathon_2025::team_captain::HackathonTeamCaptain2025Insertable, prelude::*,
};
use diesel::PgConnection;

pub fn new(pool: &DbPool, data: HackathonTeamCaptain2025Insertable) -> Result<i32, ApiError> {
    diesel::insert_into(crate::diesel::schema::hackathon_team_captain_2025::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_team_captain_2025::team_id)
        .get_result::<i32>(&mut get_connection(pool)?)
        .map_err(|err| ApiError::FailedToInsertUser(err.to_string()))
}

pub fn new_tx(
    tx: &mut PgConnection,
    data: HackathonTeamCaptain2025Insertable,
) -> Result<i32, ApiError> {
    diesel::insert_into(crate::diesel::schema::hackathon_team_captain_2025::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_team_captain_2025::team_id)
        .get_result::<i32>(tx)
        .map_err(|err| ApiError::FailedToInsertUser(err.to_string()))
}
