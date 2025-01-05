use crate::data::admin_match::AdminMatch;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Insertable;
use crate::diesel::schema::hackathon_university_2024::dsl::*;
use crate::error::api_error::ApiError;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{put, State};

#[put(
    "/hackathon_2024/university/update/<university_id>",
    data = "<university_data>"
)]
pub async fn update(
    db_pool: &State<DbPool>,
    university_id: i32,
    admin_match: AdminMatch,
    university_data: Json<HackathonUniversity2024Insertable<'_>>,
) -> Result<String, ApiError> {
    admin_match.check_admin()?;
    let university = university_data.into_inner();
    let mut db_connection = get_connection(db_pool).map_err(|_| ApiError::InternalServerError)?;

    let rows_affected = diesel::update(hackathon_university_2024.filter(id.eq(university_id)))
        .set((
            name.eq(university.name),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut db_connection)
        .map_err(|err| {
            log::error!("Error updating hackathon_university_2024: {:?}", err);
            ApiError::DatabaseErrorResult(err)
        })?;

    if rows_affected == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(format!(
        "Successfully updated hackathon_university_2024 with id: {}",
        university_id
    )
    .to_owned())
}
