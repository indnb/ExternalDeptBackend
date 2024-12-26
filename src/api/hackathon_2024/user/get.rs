use crate::api::hackathon_2024::user::local::create_user;
use crate::data::admin_match::AdminMatch;
use crate::data::claims::Claims;
use crate::diesel::database_diesel::{get_connection, DbPool};
use crate::diesel::models::hackathon_2024::hackathon_user_2024::HackathonUser2024Queryable;
use crate::diesel::schema::hackathon_user_2024::dsl::hackathon_user_2024;
use crate::diesel::schema::hackathon_user_2024::id;
use crate::error::api_error::ApiError;
use crate::utils::security::decoded_data;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use rocket::{get, State};

#[get("/hackathon_2024/user/confirm_new_user?<jwt_token>")]
pub async fn confirm_new_user(
    db_pool: &State<DbPool>,
    jwt_token: String,
) -> Result<String, ApiError> {
    let decoded = decoded_data(&jwt_token)?;
    create_user(db_pool, decoded.claims)
}

#[allow(dead_code)]
#[get("/hackathon_2024/user/get_authorization_user")]
pub async fn get_authorization_user(
    db_pool: &State<DbPool>,
    claims: Claims,
) -> Result<Json<HackathonUser2024Queryable>, ApiError> {
    let mut connection = get_connection(db_pool)?;
    let user: Result<HackathonUser2024Queryable, diesel::result::Error> = hackathon_user_2024
        .filter(id.eq(claims.sub))
        .first::<HackathonUser2024Queryable>(&mut connection);
    match user {
        Ok(user) => Ok(Json(user)),
        Err(diesel::result::Error::NotFound) => {
            Err(ApiError::Unauthorized("Token or mismatched".to_string()))
        }
        Err(e) => Err(ApiError::DatabaseError(e)),
    }
}

#[get("/hackathon_2024/user/get_all")]
pub async fn get_all(
    db_pool: &State<DbPool>,
    admin_match: AdminMatch,
) -> Result<Json<Vec<HackathonUser2024Queryable>>, ApiError> {
    admin_match.check_admin()?;
    let mut db_connection = get_connection(db_pool)?;
    let results = hackathon_user_2024
        .load::<HackathonUser2024Queryable>(&mut db_connection)
        .map_err(ApiError::DatabaseError)?;

    Ok(Json(results))
}
