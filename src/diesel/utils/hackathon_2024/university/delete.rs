use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_university_2024::dsl::hackathon_university_2024;

pub fn by_id(db_pool: &DbState, id: i32) -> Result<usize, ApiError> {
    diesel::delete(
        hackathon_university_2024
            .filter(crate::diesel::schema::hackathon_university_2024::id.eq(id)),
    )
    .execute(&mut get_connection(db_pool)?)
    .map_err(|err| {
        log::error!("Error deleting university from hackathon_university_2024 with id {id}");
        ApiError::DatabaseErrorResult(err)
    })
}
