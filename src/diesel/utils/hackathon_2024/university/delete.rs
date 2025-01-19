use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_university_2024::dsl::hackathon_university_2024;
use crate::diesel::schema::hackathon_university_2024::id;

pub fn by_id(db_pool: &DbState, delete_id: i32) -> Result<i32, ApiError> {
    diesel::delete(hackathon_university_2024.filter(id.eq(delete_id)))
        .returning(id)
        .get_result(&mut get_connection(db_pool)?)
        .map_err(|err| {
            error!("Error deleting university from hackathon_university_2024 with id {delete_id}");
            err.into()
        })
}
