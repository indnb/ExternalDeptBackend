use crate::dto::response::hackathon_2024::university::{
    University, VecUniversity, UNIVERSITY_ALL_CACHED,
};
use crate::utils::prelude_api::*;
use rocket::get;

#[utoipa::path(
    get,
    path = "/api/hackathon_2024/university/all",
    tag = "Hackathon University 2024",
    operation_id = "get_all_university",
    responses(
        (status = 200, description = "All university fetched successfully", body = Vec<University>),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
)]
#[get("/hackathon_2024/university/all")]
pub async fn all() -> Result<Json<VecUniversity>, ApiError> {
    let all: Vec<_> = UNIVERSITY_ALL_CACHED
        .read()
        .await
        .clone()
        .into_values()
        .collect();

    Ok(Json(VecUniversity(all)))
}

#[utoipa::path(
    get,
    path = "/api/hackathon_2024/university/by_id/{id}",
    tag = "Hackathon University 2024",
    operation_id = "get_university_by_id",
    params(
        ("id" = i32, Path, description = "ID of the university to fetch")
    ),
    responses(
        (status = 200, description = "Single university fetched by id successfully", body = University),
        (status = 401, description = "Unauthorized error"),
        (status = 500, description = "Database error"),
    ),
)]
#[get("/hackathon_2024/university/by_id/<id>")]
pub async fn by_id(id: i32) -> Result<Json<University>, ApiError> {
    Ok(Json(University(
        UNIVERSITY_ALL_CACHED.read().await.get(&id).unwrap().clone(),
    )))
}
