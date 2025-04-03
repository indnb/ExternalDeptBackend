use crate::diesel::models::hackathon_2025::university::HackathonUniversity2025Insertable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_university_2025::dsl::hackathon_university_2025;
use crate::diesel::schema::hackathon_university_2025::{id, name, name_eng, updated_at};

pub fn by_id(
    db_pool: &State<DbPool>,
    university_id: i32,
    data: HackathonUniversity2025Insertable,
) -> Result<i32, ApiError> {
    diesel::update(hackathon_university_2025.filter(id.eq(university_id)))
        .set((
            name.eq(data.name),
            name_eng.eq(data.name_eng),
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
