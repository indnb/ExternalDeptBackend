use crate::diesel::prelude::get_connection;
use crate::dto::response::hackathon_2025::user::User;
use crate::dto::response::hackathon_2025::user::VecUser;
use crate::utils::prelude_api::*;
use rocket::get;

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/user/all",
    tag = "Hackathon User 2025",
    operation_id = "get_all_user",
    responses(
        (status = 200, description = "All user fetched successfully", body = Vec<User>),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2025/user/all")]
pub async fn all(pool: &DbState, admin_match: AdminAuthData) -> Result<Json<VecUser>, ApiError> {
    admin_match.check_admin()?;
    Ok(Json(VecUser(
        crate::diesel::utils::hackathon_2025::user::fetch::all(&mut get_connection(pool)?)?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/user/by_id/{id}",
    tag = "Hackathon User 2025",
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
#[get("/hackathon_2025/user/by_id/<id>")]
pub async fn by_id(
    pool: &DbState,
    id: i32,
    admin_match: AdminAuthData,
) -> Result<Json<User>, ApiError> {
    admin_match.check_admin()?;
    Ok(Json(User(
        crate::diesel::utils::hackathon_2025::user::fetch::by_id(&mut get_connection(pool)?, id)?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/user/by_university/{id}",
    tag = "Hackathon User 2024",
    operation_id = "get_user_by_university",
    params(
        ("id" = i32, Path, description = "ID of the user`s university to fetch")
    ),
    responses(
        (status = 200, description = "User fetched successfully", body = Vec<User>),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2025/user/by_university/<id>")]
pub async fn by_university(
    db_pool: &DbState,
    id: i32,
    admin_match: AdminAuthData,
) -> Result<Json<VecUser>, ApiError> {
    admin_match.check_admin()?;
    Ok(Json(VecUser(
        crate::diesel::utils::hackathon_2025::user::fetch::by_university(db_pool, id)?,
    )))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2025/user/by_team/{id}",
    tag = "Hackathon User 2025",
    operation_id = "get_user_by_team",
    params(
        ("id" = i32, Path, description = "ID of the user`s team to fetch")
    ),
    responses(
        (status = 200, description = "User fetched successfully", body = Vec<User>),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/hackathon_2025/user/by_team/<id>")]
pub async fn by_team(db_pool: &DbState, id: i32) -> Result<Json<VecUser>, ApiError> {
    Ok(Json(VecUser(
        crate::diesel::utils::hackathon_2025::user::fetch::by_team(db_pool, id)?,
    )))
}
