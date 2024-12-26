use crate::data::hackathon_2024::user::UserJwt;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_2024::hackathon_user_2024::HackathonUser2024Insertable;
use crate::error::api_error::ApiError;
use diesel::RunQueryDsl;
use rocket::{info, State};
use crate::utils::validation::data;

pub fn create_user(db_pool: &State<DbPool>, new_user: UserJwt) -> Result<String, ApiError> {
    let mut db_connection = get_connection(db_pool)?;
    let mut new_user = HackathonUser2024Insertable {
        first_name: new_user.first_name.as_str(),
        last_name: new_user.last_name.as_str(),
        email: new_user.email.as_str(),
        phone: new_user.phone.as_str(),
        category: new_user.category,
        university: new_user.university,
    };
    data::hackathon_2024::user::field(&mut new_user)?;

    let user_id = diesel::insert_into(crate::diesel::schema::hackathon_user_2024::table)
        .values(new_user)
        .returning(crate::diesel::schema::hackathon_user_2024::id)
        .get_result::<i32>(&mut db_connection)
        .map_err(|err| {
            log::error!("Error inserting user: {:?}", err);
            ApiError::DatabaseError(err)
        })?;

    info!("Successfully inserted a new user with ID: {}", user_id);
    Ok(format!("User created successfully with ID: {}", user_id))
}
