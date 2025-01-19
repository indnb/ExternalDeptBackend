use crate::diesel::models::hackathon_2024::user::HackathonUser2024Queryable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_user_2024::dsl::hackathon_user_2024;
use crate::diesel::schema::hackathon_user_2024::id;

pub fn all(db_pool: &DbState) -> Result<Vec<HackathonUser2024Queryable>, ApiError> {
    hackathon_user_2024
        .load::<HackathonUser2024Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| err.into())
}
pub fn by_id(db_pool: &DbState, user_id: i32) -> Result<HackathonUser2024Queryable, ApiError> {
    hackathon_user_2024
        .filter(id.eq(user_id))
        .first::<HackathonUser2024Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| err.into())
}
