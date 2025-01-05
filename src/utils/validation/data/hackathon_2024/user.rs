use crate::diesel::models::hackathon_2024::user::HackathonUser2024Insertable;
use crate::error::api_error::ApiError;
use crate::utils::validation::data::fields::{check_email, check_name, check_phone};

pub fn field(new_user: &HackathonUser2024Insertable) -> Result<(), ApiError> {
    check_email(
        new_user.email.as_str(),
        format!("Email don't correct {}", new_user.email.as_str()).as_str(),
    )?;
    check_phone(
        new_user.phone.as_str(),
        format!("Phone don't correct {}", new_user.phone.as_str()).as_str(),
    )?;
    check_name(
        new_user.first_name.as_str(),
        20,
        format!(
            "First name length greater {} symbol",
            new_user.first_name.as_str()
        )
        .as_str(),
    )?;
    check_name(
        new_user.last_name.as_str(),
        20,
        format!(
            "Lastname name length greater {} symbol",
            new_user.first_name.as_str()
        )
        .as_str(),
    )?;
    Ok(())
}
