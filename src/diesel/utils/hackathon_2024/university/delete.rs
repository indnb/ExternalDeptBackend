use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_university_2024::dsl::hackathon_university_2024;
use crate::diesel::schema::hackathon_university_2024::id;

pub fn by_id(db_pool: &DbState, delete_id: i32) -> Result<i32, ApiError> {
    diesel::delete(hackathon_university_2024.filter(id.eq(delete_id)))
        .returning(id)
        .get_result(&mut get_connection(db_pool)?)
        .map(|data| {
            if data == 0 {
                Err(ApiError::FailedToDeleteUniversityById(
                    "University not found".to_string(),
                ))
            } else {
                Ok(data)
            }
        })
        .map_err(|err| ApiError::FailedToDeleteUniversityById(err.to_string()))
        .and_then(|data| data)
}
