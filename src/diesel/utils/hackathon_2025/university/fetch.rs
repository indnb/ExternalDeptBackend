use crate::diesel::models::hackathon_2025::university::HackathonUniversity2025Queryable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_university_2025::dsl::hackathon_university_2025;
use std::collections::HashMap;

pub fn all(
    db_pool: &State<DbPool>,
) -> Result<HashMap<i32, HackathonUniversity2025Queryable>, ApiError> {
    hackathon_university_2025
        .load::<HackathonUniversity2025Queryable>(&mut get_connection(db_pool)?)
        .map(|data| data.into_iter().map(|data| (data.id, data)).collect())
        .map_err(|err| ApiError::FailedToGetAllUniversities(err.to_string()))
}
