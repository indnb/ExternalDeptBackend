use crate::api::user::local::create_user;
use crate::data::claims::Claims;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::users_data::users::UserQueryable;
use crate::diesel::models::users_data::users_role::UserRoleEnum;
use crate::diesel::schema::users::dsl::*;
use crate::error::api_error::ApiError;
use crate::utils::security::decoded_data;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use rocket::{get, State};

#[get("/user/confirm_new_user?<jwt_token>")]
pub async fn confirm_new_user(
    db_pool: &State<DbPool>,
    jwt_token: String,
) -> Result<String, ApiError> {
    let decoded = decoded_data(&jwt_token)?;

    create_user(db_pool, decoded.claims).await
}

#[get("/user/get_role")]
pub async fn get_role(
    db_pool: &State<DbPool>,
    claims: Claims,
) -> Result<Json<UserRoleEnum>, ApiError> {
    let mut connection = get_connection(db_pool)?;

    let user_id: i32 = claims.sub;

    let user_role: Result<UserRoleEnum, diesel::result::Error> = users
        .filter(id.eq(user_id))
        .select(role)
        .first::<UserRoleEnum>(&mut connection);

    match user_role {
        Ok(user_role) => Ok(Json(user_role)),
        Err(diesel::result::Error::NotFound) => {
            Err(ApiError::Unauthorized("Claims mismatched".to_string()))
        }
        Err(_) => Err(ApiError::InternalServerError),
    }
}
#[get("/user/get_authorization_user")]
pub async fn get_authorization_user(
    db_pool: &State<DbPool>,
    claims: Claims,
) -> Result<Json<UserQueryable>, ApiError> {
    let mut connection = get_connection(db_pool)?;
    let user: Result<UserQueryable, diesel::result::Error> =
        users
            .filter(id.eq(claims.sub))
            .first::<UserQueryable>(&mut connection);
    match user {
        Ok(user) => Ok(Json(user)),
        Err(diesel::result::Error::NotFound) => {
            Err(ApiError::Unauthorized("Token or mismatched".to_string()))
        }
        Err(e) => Err(ApiError::DatabaseError(e)),
    }
}
