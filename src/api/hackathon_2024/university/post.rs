use crate::data::admin_match::AdminMatch;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Insertable as University;
use crate::error::api_error::ApiError;
use diesel::RunQueryDsl;
use rocket::serde::json::Json;
use rocket::{post, State};

#[post("/hackathon_2024/university/create", data = "<university_data>")]
pub async fn create(
    db_pool: &State<DbPool>,
    university_data: Json<University<'_>>,
    admin_match: AdminMatch,
) -> Result<String, ApiError> {
    admin_match.check_admin()?;

    let mut db_connection = get_connection(db_pool)?;
    let university = university_data.into_inner();

    let university_id =
        diesel::insert_into(crate::diesel::schema::hackathon_university_2024::table)
            .values(university)
            .returning(crate::diesel::schema::hackathon_university_2024::id)
            .get_result::<i32>(&mut db_connection)
            .map_err(|err| {
                log::error!("Error inserting university with id - {:?}", err);
                ApiError::DatabaseErrorResult(err)
            })?;

    Ok(format!("Succeed insert new university with id - {}", university_id).to_owned())
}
