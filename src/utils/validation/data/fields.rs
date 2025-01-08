use crate::error::api_error::ApiError;
use crate::utils::validation::validation_string::Validate;

pub fn check_email(email: impl AsRef<str>, error_message: impl AsRef<str>) -> Result<(), ApiError> {
    if email.as_ref().is_email() {
        Ok(())
    } else {
        Err(ApiError::ValidationError(error_message.as_ref().to_owned()))
    }
}

pub fn check_name(
    name: impl AsRef<str>,
    length: usize,
    error_message: impl AsRef<str>,
) -> Result<(), ApiError> {
    if name.as_ref().less_for(length) {
        Ok(())
    } else {
        Err(ApiError::ValidationError(error_message.as_ref().to_owned()))
    }
}

pub fn check_phone(phone: impl AsRef<str>, error_message: impl AsRef<str>) -> Result<(), ApiError> {
    if phone.as_ref().is_phone() {
        Ok(())
    } else {
        Err(ApiError::ValidationError(error_message.as_ref().to_owned()))
    }
}

pub fn check_password(
    password: impl AsRef<str>,
    length: usize,
    error_message: impl AsRef<str>,
) -> Result<(), ApiError> {
    if password.as_ref().is_password(length) {
        Ok(())
    } else {
        Err(ApiError::ValidationError(error_message.as_ref().to_owned()))
    }
}
