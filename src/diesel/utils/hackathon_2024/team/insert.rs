use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Insertable;
use crate::diesel::prelude::*;

pub fn new(db_pool: &DbState, data: HackathonTeam2024Insertable) -> Result<i32, ApiError> {
    let mut db_connection = get_connection(db_pool)?;
    diesel::insert_into(crate::diesel::schema::hackathon_team_2024::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_team_2024::id)
        .get_result::<i32>(&mut db_connection)
        .map_err(|err| {
            error!("Error inserting hackathon 2024 team with id - {err:?}");
            err.into()
        })
}
