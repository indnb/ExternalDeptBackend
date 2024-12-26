use crate::diesel::models::hackathon_2024::hackathon_user_2024::HackathonUser2024Insertable;
use crate::error::api_error::ApiError;
use crate::utils::validation::validation_string::Validate;

pub fn field(new_user: &mut HackathonUser2024Insertable) -> Result<(), ApiError> {
    if !new_user.email.is_email() {
        return Err(ApiError::ValidationError(
            format!("Email don't correct {}", new_user.email).to_string(),
        ));
    }

    if !new_user.phone.is_phone() {
        return Err(ApiError::ValidationError(
            format!("Phone don't correct {}", new_user.phone).to_string(),
        ));
    }

    let name_length = 20;

    if !new_user.first_name.less_for(name_length) || !new_user.last_name.less_for(name_length) {
        return Err(ApiError::ValidationError(
            format!("First name or last name greater for {} symbol", name_length).to_string(),
        ));
    }
    Ok(())
}
