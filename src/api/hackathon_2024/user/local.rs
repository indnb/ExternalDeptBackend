use crate::models::hackathon_2024::user::UserJwt;
use crate::utils::prelude_api::*;
use crate::utils::validation::data;

pub fn create_user_by_jwt(db_pool: &DbState, new_user: UserJwt) -> Result<(), ApiError> {
    let data = new_user.into();
    data::hackathon_2024::user::field(&data)?;

    let user_id = crate::diesel::utils::hackathon_2024::user::insert::new(db_pool, data)?;

    info!("Successfully inserted a new hackathon_user_2024 with ID: {user_id}");

    Ok(())
}
