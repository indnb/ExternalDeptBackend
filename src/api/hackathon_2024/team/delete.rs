use crate::data::admin_match::AdminMatch;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::schema::hackathon_team_2024::dsl::*;
use crate::diesel::schema::hackathon_team_2024::id;
use crate::error::api_error::ApiError;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use rocket::{delete, State};

#[delete("/hackathon_2024/team/by_id/<team_id>")]
pub async fn by_id(
    db_pool: &State<DbPool>,
    team_id: i32,
    admin_match: AdminMatch,
) -> Result<String, ApiError> {
    admin_match.check_admin()?;

    let mut db_connection = get_connection(db_pool)?;

    diesel::delete(hackathon_team_2024)
        .filter(id.eq(team_id))
        .execute(&mut db_connection)
        .map_err(|err| {
            log::error!("Error deleting hackathon_university_2024: {:?}", err);
            ApiError::DatabaseErrorResult(err)
        })?;

    Ok(format!(
        "Successfully deleted team from hackathon_team_2024 with id {team_id}"
    ))
}
