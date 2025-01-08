use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Queryable;
use crate::utils::prelude_api::*;
use rocket::get;

#[get("/hackathon_2024/team/all")]
pub async fn all(db_pool: &DbState) -> Result<Json<Vec<HackathonTeam2024Queryable>>, ApiError> {
    Ok(Json(
        crate::diesel::utils::hackathon_2024::team::fetch::all(db_pool)?,
    ))
}

#[get("/hackathon_2024/team/by_id/<id>")]
pub async fn by_id(
    db_pool: &DbState,
    id: i32,
) -> Result<Json<HackathonTeam2024Queryable>, ApiError> {
    Ok(Json(
        crate::diesel::utils::hackathon_2024::team::fetch::by_id(db_pool, id)?,
    ))
}
