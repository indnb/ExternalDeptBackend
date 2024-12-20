use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::users_data::users::InsertableUser;
use crate::error::api_error::ApiError;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{info, post, State};
#[post("/user", data = "<user_data>")]
pub async fn create_user(
    db_pool: &State<DbPool>,
    user_data: Json<InsertableUser<'_>>,
) -> Result<Json<String>, ApiError> {
    let mut db_connection = get_connection(db_pool)?;

    let new_user = user_data.into_inner();

    let user_id = diesel::insert_into(crate::diesel::schema::users::table)
        .values(new_user)
        .returning(crate::diesel::schema::users::id)
        .get_result::<i32>(&mut db_connection)
        .map_err(|err| {
            log::error!("Error inserting user: {:?}", err);
            ApiError::DatabaseError(err)
        })?;

    info!("Successfully inserted a new user with ID: {}", user_id);
    Ok(Json(format!(
        "User created successfully with ID: {}",
        user_id
    )))
}
