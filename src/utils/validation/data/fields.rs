use crate::error::api_error::ApiError;
use crate::utils::validation::validation_string::Validate;

#[allow(dead_code)]
pub fn check_email(email: impl AsRef<str>, error_message: impl AsRef<str>) -> Result<(), ApiError> {
    if email.as_ref().is_email() {
        Ok(())
    } else {
        Err(ApiError::InvalidEmail(error_message.as_ref().to_owned()))
    }
}

pub fn check_nickname_tg(
    nickname_tg: impl AsRef<str>,
    error_message: impl AsRef<str>,
) -> Result<(), ApiError> {
    if nickname_tg.as_ref().is_nickname_tg() {
        Ok(())
    } else {
        Err(ApiError::InvalidTelegramNickname(
            error_message.as_ref().to_owned(),
        ))
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
        Err(ApiError::InvalidName(error_message.as_ref().to_owned()))
    }
}

pub fn check_phone(phone: impl AsRef<str>, error_message: impl AsRef<str>) -> Result<(), ApiError> {
    if phone.as_ref().is_phone() {
        Ok(())
    } else {
        Err(ApiError::InvalidPhoneNumber(
            error_message.as_ref().to_owned(),
        ))
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
        Err(ApiError::InvalidPassword(error_message.as_ref().to_owned()))
    }
}
