use crate::data::admin_match::AdminMatch;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_2024::hackathon_user_2024::HackathonUser2024Insertable;
use crate::diesel::schema::hackathon_user_2024::dsl::*;
use crate::error::api_error::ApiError;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{put, State};

#[put("/hackathon_2024/user/update/<user_id>", data = "<user_data>")]
pub async fn update(
    db_pool: &State<DbPool>,
    user_data: Json<HackathonUser2024Insertable<'_>>,
    user_id: i32,
    admin_match: AdminMatch,
) -> Result<String, ApiError> {
    admin_match.check_admin()?;

    let new_user = user_data.into_inner();

    let mut db_connection = get_connection(db_pool)?;

    let rows_affected = diesel::update(hackathon_user_2024.filter(id.eq(user_id)))
        .set((
            first_name.eq(new_user.first_name),
            last_name.eq(new_user.last_name),
            email.eq(new_user.email),
            phone.eq(new_user.phone),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut db_connection)
        .map_err(|err| {
            log::error!("Error updating hackathon_user_2024: {:?}", err);
            ApiError::DatabaseErrorResult(err)
        })?;

    if rows_affected == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(format!(
        "Successfully updated hackathon_user_2024 with email: {}",
        new_user.email
    )
    .to_owned())
}
