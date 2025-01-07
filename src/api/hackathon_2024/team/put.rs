use crate::data::admin_match::AdminMatch;
use crate::data::hackathon_2024::team::TeamUpdateData;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::schema::hackathon_team_2024::dsl::hackathon_team_2024;
use crate::diesel::schema::hackathon_team_2024::{category, email, id, name, updated_at};
use crate::error::api_error::ApiError;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use rocket::serde::json::Json;
use rocket::{put, State};

#[put("/hackathon_2024/team/by_id", data = "<data>")]
pub async fn by_id(
    db_pool: &State<DbPool>,
    data: Json<TeamUpdateData>,
    admin_match: AdminMatch,
) -> Result<String, ApiError> {
    admin_match.check_admin()?;
    let data = data.into_inner();

    let mut db_connection = get_connection(db_pool)?;

    diesel::update(hackathon_team_2024)
        .filter(id.eq(data.id))
        .set((
            name.eq(data.name.clone()),
            category.eq(data.category),
            email.eq(data.email.clone()),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut db_connection)
        .map_err(|err| {
            log::error!("Error updating hackathon_university_2024, bellow error");
            ApiError::DatabaseErrorResult(err)
        })?;

    Ok(format!(
        "Successfully updating team from hackathon_team_2024 with data: {data:?}"
    ))
}
