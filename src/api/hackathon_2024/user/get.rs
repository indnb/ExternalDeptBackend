use crate::api::hackathon_2024::user::local::create_user_by_jwt;
use crate::dto::response::hackathon_2024::user::User;
use crate::dto::response::hackathon_2024::user::VecUser;
use crate::middleware::claims::Claims;
use crate::utils::prelude_api::*;
use crate::utils::security::decoded_data;
use rocket::get;

#[allow(dead_code)]
#[get("/hackathon_2024/user/confirm_new_user?<jwt_token>")]
pub async fn confirm_new_user(db_pool: &DbState, jwt_token: String) -> Result<(), ApiError> {
    create_user_by_jwt(db_pool, decoded_data(&jwt_token)?.claims)
}

#[allow(dead_code)]
#[utoipa::path(
    get,
    path = "/api/hackathon_2024/user/authorization_user",
    tag = "Hackathon User 2024",
    operation_id = "get_authorization_user",
    responses(
        (status = 200, description = "User fetched successfully", body = User),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2024/user/authorization_user")]
pub async fn authorization_user(db_pool: &DbState, claims: Claims) -> Result<Json<User>, ApiError> {
    Ok(Json(User(
        crate::diesel::utils::hackathon_2024::user::fetch::by_id(db_pool, claims.sub)?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2024/user/all",
    tag = "Hackathon User 2024",
    operation_id = "get_all_user",
    responses(
        (status = 200, description = "All user fetched successfully", body = VecUser),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2024/user/all")]
pub async fn all(db_pool: &DbState, admin_match: AdminAuthData) -> Result<Json<VecUser>, ApiError> {
    admin_match.check_admin()?;
    Ok(Json(VecUser(
        crate::diesel::utils::hackathon_2024::user::fetch::all(db_pool)?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2024/user/by_id/{id}",
    tag = "Hackathon User 2024",
    operation_id = "get_user_by_id",
    params(
        ("id" = i32, Path, description = "ID of the user to fetch")
    ),
    responses(
        (status = 200, description = "User fetched successfully", body = User),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2024/user/by_id/<id>")]
pub async fn by_id(
    db_pool: &DbState,
    id: i32,
    admin_match: AdminAuthData,
) -> Result<Json<User>, ApiError> {
    admin_match.check_admin()?;
    Ok(Json(User(
        crate::diesel::utils::hackathon_2024::user::fetch::by_id(db_pool, id)?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2024/user/by_university/{id}",
    tag = "Hackathon User 2024",
    operation_id = "get_user_by_university",
    params(
        ("id" = i32, Path, description = "ID of the user`s university to fetch")
    ),
    responses(
        (status = 200, description = "User fetched successfully", body = VecUser),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2024/user/by_university/<id>")]
pub async fn by_university(
    db_pool: &DbState,
    id: i32,
    admin_match: AdminAuthData,
) -> Result<Json<VecUser>, ApiError> {
    admin_match.check_admin()?;
    Ok(Json(VecUser(
        crate::diesel::utils::hackathon_2024::user::fetch::by_university(db_pool, id)?,
    )))
}

// do swagger doc get user by team
#[utoipa::path(
    get,
    path = "/api/hackathon_2024/user/by_team/{id}",
    tag = "Hackathon User 2024",
    operation_id = "get_user_by_team",
    params(
        ("id" = i32, Path, description = "ID of the user`s team to fetch")
    ),
    responses(
        (status = 200, description = "User fetched successfully", body = VecUser),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2024/user/by_team/<id>")]
pub async fn by_team(db_pool: &DbState, id: i32) -> Result<Json<VecUser>, ApiError> {
    Ok(Json(VecUser(
        crate::diesel::utils::hackathon_2024::user::fetch::by_team(db_pool, id)?,
    )))
}
