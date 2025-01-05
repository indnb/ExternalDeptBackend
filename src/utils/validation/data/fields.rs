use crate::error::api_error::ApiError;
use crate::utils::validation::validation_string::Validate;

pub fn check_email(email: &str, error_message: &str) -> Result<(), ApiError> {
    if email.is_email() {
        Ok(())
    } else {
        Err(ApiError::ValidationError(error_message.to_owned()))
    }
}
pub fn check_name(name: &str, lenght: usize, error_message: &str) -> Result<(), ApiError> {
    if name.less_for(lenght) {
        Ok(())
    } else {
        Err(ApiError::ValidationError(error_message.to_owned()))
    }
}
pub fn check_phone(phone: &str, error_message: &str) -> Result<(), ApiError> {
    if phone.is_phone() {
        Ok(())
    } else {
        Err(ApiError::ValidationError(error_message.to_owned()))
    }
}
pub fn check_password(password: &str, length: usize, error_message: &str) -> Result<(), ApiError> {
    if password.is_password(length) {
        Ok(())
    } else {
        Err(ApiError::ValidationError(error_message.to_owned()))
    }
}
