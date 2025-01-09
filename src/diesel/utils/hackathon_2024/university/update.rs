use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Insertable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_university_2024::dsl::hackathon_university_2024;
use crate::diesel::schema::hackathon_university_2024::{id, name, updated_at};

pub fn by_id(
    db_pool: &State<DbPool>,
    university_id: i32,
    data: HackathonUniversity2024Insertable,
) -> Result<usize, ApiError> {
    diesel::update(hackathon_university_2024.filter(id.eq(university_id)))
        .set((
            name.eq(data.name),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut get_connection(db_pool)?)
        .map_err(|err| {
            error!("Error updating hackathon_university_2024");
            ApiError::DatabaseErrorResult(err)
        })
}
