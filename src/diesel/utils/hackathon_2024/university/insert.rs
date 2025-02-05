use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Insertable;
use crate::diesel::prelude::*;

pub fn new(
    db_pool: &State<DbPool>,
    data: HackathonUniversity2024Insertable,
) -> Result<i32, ApiError> {
    let mut db_connection = get_connection(db_pool)?;
    diesel::insert_into(crate::diesel::schema::hackathon_university_2024::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_university_2024::id)
        .get_result::<i32>(&mut db_connection)
        .map_err(|err| ApiError::FailedToInsertUniversity(err.to_string()))
}

pub fn new_by_vec(
    db_pool: &State<DbPool>,
    data: Vec<HackathonUniversity2024Insertable>,
) -> Result<i32, ApiError> {
    let mut db_connection = get_connection(db_pool)?;
    diesel::insert_into(crate::diesel::schema::hackathon_university_2024::table)
        .values(data)
        .returning(crate::diesel::schema::hackathon_university_2024::id)
        .get_result::<i32>(&mut db_connection)
        .map_err(|err| ApiError::FailedToInsertUniversityFromVec(err.to_string()))
}
