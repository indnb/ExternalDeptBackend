use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_user_2024::dsl::hackathon_user_2024;
use crate::diesel::schema::hackathon_user_2024::id;

pub fn by_id(db_pool: &DbState, user_id: i32) -> Result<usize, ApiError> {
    diesel::delete(hackathon_user_2024.filter(id.eq(user_id)))
        .execute(&mut get_connection(db_pool)?)
        .map_err(|err| {
            error!("Error deleting hackathon_user_2024");
            err.into()
        })
}
