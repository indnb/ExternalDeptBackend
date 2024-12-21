use crate::data::user::UserJwt;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::users_data::users::UserInsertable;
use crate::error::api_error::ApiError;
use crate::utils::validation;
use diesel::RunQueryDsl;
use rocket::{info, State};
use crate::utils::security::hashing_data;

pub async fn create_user(db_pool: &State<DbPool>, new_user: UserJwt) -> Result<String, ApiError> {
    let mut db_connection = get_connection(db_pool)?;
    let mut new_user = UserInsertable {
        first_name: new_user.first_name.as_str(),
        last_name: new_user.last_name.as_str(),
        password: new_user.password.as_str(),
        email: new_user.email.as_str(),
        phone: new_user.phone.as_str(),
        role: new_user.role,
    };
    let _ = validation::data::user::field(&mut new_user)?;

    let hashed_password = &hashing_data(&mut new_user.password.to_string())?;
    new_user.password = hashed_password;

    let user_id = diesel::insert_into(crate::diesel::schema::users::table)
        .values(new_user)
        .returning(crate::diesel::schema::users::id)
        .get_result::<i32>(&mut db_connection)
        .map_err(|err| {
            log::error!("Error inserting user: {:?}", err);
            ApiError::DatabaseError(err)
        })?;

    info!("Successfully inserted a new user with ID: {}", user_id);
    Ok(format!("User created successfully with ID: {}", user_id))
}