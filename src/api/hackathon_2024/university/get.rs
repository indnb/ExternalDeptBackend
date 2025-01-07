use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Queryable;
use crate::error::api_error::ApiError;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{get, State};

use crate::diesel::schema::hackathon_university_2024::dsl::*;

#[get("/hackathon_2024/university/all")]
pub async fn all(
    db_pool: &State<DbPool>,
) -> Result<Json<Vec<HackathonUniversity2024Queryable>>, ApiError> {
    let mut db_connection = get_connection(db_pool).map_err(|_| ApiError::InternalServerError)?;

    let result = hackathon_university_2024
        .load::<HackathonUniversity2024Queryable>(&mut db_connection)
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(result))
}

#[get("/hackathon_2024/university/by_id/<university_id>")]
pub async fn by_id(
    db_pool: &State<DbPool>,
    university_id: i32,
) -> Result<Json<HackathonUniversity2024Queryable>, ApiError> {
    let mut db_connection = get_connection(db_pool).map_err(|_| ApiError::InternalServerError)?;

    let result = hackathon_university_2024
        .filter(id.eq(university_id))
        .first::<HackathonUniversity2024Queryable>(&mut db_connection)
        .map_err(|_| ApiError::NotFound)?;

    Ok(Json(result))
}
