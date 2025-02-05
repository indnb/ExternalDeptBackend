use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Insertable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_university_2024::dsl::hackathon_university_2024;
use crate::diesel::schema::hackathon_university_2024::{id, name, updated_at};

pub fn by_id(
    db_pool: &State<DbPool>,
    university_id: i32,
    data: HackathonUniversity2024Insertable,
) -> Result<i32, ApiError> {
    diesel::update(hackathon_university_2024.filter(id.eq(university_id)))
        .set((
            name.eq(data.name),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .returning(id)
        .get_result(&mut get_connection(db_pool)?)
        .map(|data| {
            if data == 0 {
                Err(ApiError::FailedToUpdateUniversityById(
                    "University not found".to_string(),
                ))
            } else {
                Ok(data)
            }
        })
        .map_err(|err| ApiError::FailedToUpdateUniversityById(err.to_string()))
        .and_then(|data| data)
}
