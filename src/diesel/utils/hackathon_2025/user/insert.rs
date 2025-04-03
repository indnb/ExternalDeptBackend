use diesel::PgConnection;

use crate::diesel::models::hackathon_2025::user::HackathonUser2025Insertable;
use crate::diesel::prelude::*;

pub fn new(db_pool: &DbState, data: HackathonUser2025Insertable) -> Result<i32, ApiError> {
    diesel::insert_into(crate::diesel::schema::hackathon_user_2025::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_user_2025::id)
        .get_result::<i32>(&mut get_connection(db_pool)?)
        .map_err(|err| ApiError::FailedToInsertUser(err.to_string()))
}

pub fn new_tx(tx: &mut PgConnection, data: HackathonUser2025Insertable) -> Result<i32, ApiError> {
    diesel::insert_into(crate::diesel::schema::hackathon_user_2025::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_user_2025::id)
        .get_result::<i32>(tx)
        .map_err(|err| ApiError::FailedToInsertUser(err.to_string()))
}
