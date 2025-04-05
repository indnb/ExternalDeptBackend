use diesel::PgConnection;

use crate::diesel::models::hackathon_2025::team::HackathonTeam2025Insertable;
use crate::diesel::prelude::*;

pub fn new(conn: &mut PgConnection, data: HackathonTeam2025Insertable) -> Result<i32, ApiError> {
    diesel::insert_into(crate::diesel::schema::hackathon_team_2025::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_team_2025::id)
        .get_result::<i32>(conn)
        .map_err(|err| ApiError::FailedToInsertTeam(err.to_string()))
}
