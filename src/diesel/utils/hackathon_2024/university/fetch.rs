use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Queryable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_university_2024::dsl::hackathon_university_2024;

pub fn by_id(db_pool: &DbState, id: i32) -> Result<HackathonUniversity2024Queryable, ApiError> {
    hackathon_university_2024
        .filter(crate::diesel::schema::hackathon_university_2024::columns::id.eq(id))
        .first::<HackathonUniversity2024Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| {
            error!("Error to get hackathon_university_2024 with id {id}");
            err.into()
        })
}

pub fn all(db_pool: &State<DbPool>) -> Result<Vec<HackathonUniversity2024Queryable>, ApiError> {
    hackathon_university_2024
        .load::<HackathonUniversity2024Queryable>(&mut get_connection(db_pool)?)
        .map_err(|err| {
            error!("Error to get all hackathon_university_2024");
            err.into()
        })
}
