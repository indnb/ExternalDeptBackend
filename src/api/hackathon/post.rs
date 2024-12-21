use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_data::hackathon::HackathonInsertable;
use crate::error::api_error::ApiError;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{info, post, State};

#[post("/create_hackathon", data = "<hackathon_data>")]
pub async fn create_hackathon(
    db_pool: &State<DbPool>,
    hackathon_data: Json<HackathonInsertable>,
) -> Result<Json<String>, ApiError> {
    let mut db_connection = get_connection(db_pool)?;

    let new_user = hackathon_data.into_inner();

    let hackathon_id = diesel::insert_into(crate::diesel::schema::hackathon::table)
        .values(new_user)
        .returning(crate::diesel::schema::hackathon::id)
        .get_result::<i32>(&mut db_connection)
        .map_err(|err| {
            log::error!("Error inserting hackathon: {:?}", err);
            ApiError::DatabaseError(err)
        })?;

    info!(
        "Successfully inserted a new hackathon with ID: {}",
        hackathon_id
    );
    Ok(Json(format!(
        "Hackathon created successfully with ID: {}",
        hackathon_id
    )))
}
