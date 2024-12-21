use crate::api::user::local::create_user;
use crate::data::claims::Claims;
use crate::data::user::UserJwt;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::users_data::users_role::UserRoleEnum;
use crate::diesel::schema::users::dsl::*;
use crate::error::api_error::ApiError;
use crate::utils::env_configuration::CONFIG;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::serde::json::Json;
use rocket::{get, State};

#[get("/user/confirm_new_user?<jwt_token>")]
pub async fn confirm_new_user(
    db_pool: &State<DbPool>,
    jwt_token: String,
) -> Result<String, ApiError> {
    let decoded = decode::<UserJwt>(
        &jwt_token,
        &DecodingKey::from_secret(CONFIG.get().unwrap().jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|err| ApiError::TokenDecodeError(err.to_string()))?;

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
        Err(diesel::result::Error::NotFound) => Err(ApiError::Unauthorized),
        Err(_) => Err(ApiError::InternalServerError),
    }
}
