use crate::diesel::models::hackathon_2025::user::HackathonUser2025Insertable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_user_2025::dsl::hackathon_user_2025;
use crate::diesel::schema::hackathon_user_2025::dsl::*;
use crate::diesel::schema::hackathon_user_2025::phone;
use diesel::{OptionalExtension, PgConnection};

pub fn new(
    tx: &mut PgConnection,
    data: HackathonUser2025Insertable,
    user_index: i32,
) -> Result<i32, ApiError> {
    check_dublicate(tx, &data, user_index)?;

    diesel::insert_into(hackathon_user_2025)
        .values(data)
        .returning(id)
        .get_result::<i32>(tx)
        .map_err(|err| ApiError::FailedToInsertUser(err.to_string()))
}

fn check_dublicate(
    tx: &mut PgConnection,
    data: &HackathonUser2025Insertable,
    user_index: i32,
) -> Result<(), ApiError> {
    if let Some(ref phone_val) = data.phone {
        let existing_phone = hackathon_user_2025
            .filter(phone.eq(phone_val))
            .select(id)
            .first::<i32>(tx)
            .optional()
            .map_err(|err| {
                ApiError::DublicatePhone(format!("index user {}: {}", user_index, err))
            })?;

        if existing_phone.is_some() {
            return Err(ApiError::DublicatePhone(format!(
                "index user {}",
                user_index
            )));
        }
    }

    if let Some(ref nickname_val) = data.nickname_tg {
        let existing_nickname = hackathon_user_2025
            .filter(nickname_tg.eq(nickname_val))
            .select(id)
            .first::<i32>(tx)
            .optional()
            .map_err(|err| {
                ApiError::DublicateNicknameTg(format!("index user {}: {}", user_index, err))
            })?;

        if existing_nickname.is_some() {
            return Err(ApiError::DublicateNicknameTg(format!(
                "index user {}",
                user_index
            )));
        }
    };

    Ok(())
}
