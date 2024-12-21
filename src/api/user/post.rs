use crate::data::claims::Claims;
use crate::data::user::{UserJwt, UserLoginRequest, UserLoginResponse};
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::users_data::users::{UserInsertable, UserQueryable};
use crate::diesel::schema::users::dsl::{email, users};
use crate::error::api_error::ApiError;
use crate::utils::security::{encoded_data, verify_password};
use crate::utils::validation;
use diesel::QueryDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::serde::json::Json;
use rocket::{info, post, State};

#[post("/user/try_registration", data = "<user_data>")]
pub async fn try_registration(user_data: Json<UserInsertable<'_>>) -> Result<String, ApiError> {
    let mut new_user = user_data.into_inner();

    validation::data::user::field(&mut new_user)?;

    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(5))
        .expect("Failed to compute expiration time")
        .timestamp() as usize;

    let jwt_user = UserJwt {
        first_name: new_user.first_name.to_string(),
        last_name: new_user.last_name.to_string(),
        password: new_user.password.to_string(),
        email: new_user.email.to_string(),
        phone: new_user.phone.to_string(),
        role: new_user.role,
        exp,
    };

    let token = encoded_data(&jwt_user)?;

    info!("Email has been send with token - {}", token);

    Ok(format!("Email has been send with token - {}", token).to_string())
}
#[post("/user/login_user", data = "<login_data>")]
pub async fn login_user(
    db_pool: &State<DbPool>,
    login_data: Json<UserLoginRequest>,
) -> Result<Json<UserLoginResponse>, ApiError> {
    let mut connection = get_connection(db_pool)?;
    let login_data = login_data.into_inner();

    let user: Result<UserQueryable, diesel::result::Error> = users
        .filter(email.eq(login_data.email))
        .first::<UserQueryable>(&mut connection);

    match user {
        Ok(user) => {
            if verify_password(&login_data.password, &user.password_hash)? {
                let token = encoded_data(&Claims::new(user.id))?;
                Ok(Json(UserLoginResponse {
                    id: user.id,
                    email: user.email,
                    token,
                }))
            } else {
                Err(ApiError::Unauthorized("Password mismatched".to_string()))
            }
        }
        Err(diesel::result::Error::NotFound) => {
            Err(ApiError::Unauthorized("Email mismatched".to_string()))
        }
        Err(e) => Err(ApiError::DatabaseError(e)),
    }
}
