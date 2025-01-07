use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Queryable;
use crate::diesel::schema::hackathon_team_2024::dsl::hackathon_team_2024;
use crate::diesel::schema::hackathon_team_2024::id;
use crate::error::api_error::ApiError;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{get, State};

#[get("/hackathon_2024/team/all")]
pub async fn all(
    db_pool: &State<DbPool>,
) -> Result<Json<Vec<HackathonTeam2024Queryable>>, ApiError> {
    let mut db_connection = get_connection(db_pool)?;

    let result = hackathon_team_2024
        .load::<HackathonTeam2024Queryable>(&mut db_connection)
        .map_err(ApiError::DatabaseErrorResult)?;

    Ok(Json(result))
}

#[get("/hackathon_2024/team/by_id/<team_id>")]
pub async fn by_id(
    db_pool: &State<DbPool>,
    team_id: i32,
) -> Result<Json<HackathonTeam2024Queryable>, ApiError> {
    let mut db_connection = get_connection(db_pool)?;

    let result = hackathon_team_2024
        .filter(id.eq(team_id))
        .first::<HackathonTeam2024Queryable>(&mut db_connection)
        .map_err(ApiError::DatabaseErrorResult)?;

    Ok(Json(result))
}
