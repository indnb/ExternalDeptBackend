use crate::dto::response::hackathon_2024::university::{University, VecUniversity};
use crate::utils::prelude_api::*;
use rocket::get;

#[get("/hackathon_2024/university/all")]
pub async fn all(db_pool: &DbState) -> Result<Json<VecUniversity>, ApiError> {
    Ok(Json(VecUniversity(
        crate::diesel::utils::hackathon_2024::university::fetch::all(db_pool)?,
    )))
}

#[get("/hackathon_2024/university/by_id/<id>")]
pub async fn by_id(db_pool: &DbState, id: i32) -> Result<Json<University>, ApiError> {
    Ok(Json(University(
        crate::diesel::utils::hackathon_2024::university::fetch::by_id(db_pool, id)?,
    )))
}
