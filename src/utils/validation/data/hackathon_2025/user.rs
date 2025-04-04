use crate::diesel::models::hackathon_2025::user::HackathonUser2025Insertable;
use crate::error::api_error::ApiError;
use crate::utils::validation::data::fields::{
    check_hasnt_symbol, check_name, check_nickname_tg, check_phone,
};

pub fn field(new_user: &HackathonUser2025Insertable) -> Result<(), ApiError> {
    if let Some(ref tg) = new_user.nickname_tg {
        check_nickname_tg(tg.as_str(), format!("Nickname tg don't correct {}", tg))?;
    }

    if let Some(ref phone) = new_user.phone {
        check_phone(phone.as_str(), format!("Phone don't correct {}", phone))?;
    }

    check_name(
        new_user.first_name.as_str(),
        20,
        format!("First name length greater {} symbol", new_user.first_name),
    )?;

    check_hasnt_symbol(
        new_user.first_name.as_str(),
        format!("First name has symbol {}", new_user.first_name),
    )?;

    check_name(
        new_user.last_name.as_str(),
        20,
        format!(
            "Lastname name length greater {} symbol",
            new_user.first_name
        ),
    )?;

    check_hasnt_symbol(
        new_user.last_name.as_str(),
        format!("Lastname name has symbol {}", new_user.first_name),
    )?;

    Ok(())
}
