use crate::diesel::configurator::{get_connection, DbPool};
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_team_2024::dsl::hackathon_team_2024;
use crate::error::api_error::ApiError;
use rocket::State;

pub fn by_id(db_pool: &State<DbPool>, id: i32) -> Result<usize, ApiError> {
    diesel::delete(hackathon_team_2024)
        .filter(crate::diesel::schema::hackathon_team_2024::id.eq(id))
        .execute(&mut get_connection(db_pool)?)
        .map_err(|err| {
            error!("Error deleting hackathon_university_2024 with id {id}");
            err.into()
        })
}
