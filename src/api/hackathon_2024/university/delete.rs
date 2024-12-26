use crate::data::admin_match::AdminMatch;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::schema::hackathon_university_2024::dsl::hackathon_university_2024;
use crate::diesel::schema::hackathon_university_2024::id;
use crate::error::api_error::ApiError;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use rocket::{delete, State};

#[delete("/hackathon_2024/university/delete_by_id/<university_id>")]
pub async fn delete_by_id(
    db_pool: &State<DbPool>,
    university_id: i32,
    admin_match: AdminMatch,
) -> Result<String, ApiError> {
    admin_match.check_admin()?;

    let mut db_connection = get_connection(db_pool)?;

    let rows_deleted = diesel::delete(hackathon_university_2024.filter(id.eq(university_id)))
        .execute(&mut db_connection)
        .map_err(|err| {
            log::error!("Error deleting hackathon_university_2024: {:?}", err);
            ApiError::DatabaseErrorResult(err)
        })?;

    if rows_deleted == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(format!(
        "Successfully deleted hackathon_university_2024 from hackathon 2024 with id {}",
        university_id
    )
    .to_owned())
}
