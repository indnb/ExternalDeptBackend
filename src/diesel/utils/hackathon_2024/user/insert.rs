use crate::diesel::models::hackathon_2024::user::HackathonUser2024Insertable;
use crate::diesel::prelude::*;

pub fn new(db_pool: &DbState, data: HackathonUser2024Insertable) -> Result<i32, ApiError> {
    diesel::insert_into(crate::diesel::schema::hackathon_user_2024::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_user_2024::id)
        .get_result::<i32>(&mut get_connection(db_pool)?)
        .map_err(|err| ApiError::FailedToInsertUser(err.to_string()))
}
