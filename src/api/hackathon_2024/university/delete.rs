use crate::{
    dto::response::hackathon_2024::university::update_university_cached, utils::prelude_api::*,
};
use rocket::delete;

#[utoipa::path(
    delete,
    path = "/api/hackathon_2024/university/by_id/{id}",
    tag = "Hackathon University 2024",
    operation_id = "delete_university_by_id", 
    responses(
        (status = 200, description = "University deleted successfully"),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    operation_id = "delete_university_by_id",
    params(
        ("id" = i32, Path, description = "ID of the university to delete")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[delete("/hackathon_2024/university/by_id/<id>")]
pub async fn by_id(db_pool: &DbState, id: i32, admin_match: AdminAuthData) -> Result<(), ApiError> {
    admin_match.check_admin()?;
    let id = crate::diesel::utils::hackathon_2024::university::delete::by_id(db_pool, id)?;

    update_university_cached(db_pool).await?;

    info!("Successfully deleted university from hackathon_university_2024 with id {id}");

    Ok(())
}
