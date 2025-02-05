use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Queryable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_university_2024::dsl::hackathon_university_2024;
use std::collections::HashMap;

pub fn all(
    db_pool: &State<DbPool>,
) -> Result<HashMap<i32, HackathonUniversity2024Queryable>, ApiError> {
    hackathon_university_2024
        .load::<HackathonUniversity2024Queryable>(&mut get_connection(db_pool)?)
        .map(|data| data.into_iter().map(|data| (data.id, data)).collect())
        .map_err(|err| ApiError::FailedToGetAllUniversities(err.to_string()))
}
