use crate::diesel::models::hackathon_2024::user::HackathonUser2024Queryable;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[schema(title = "HackathonUser2024Queryable", value_type = HackathonUser2024Queryable, as = HackathonUser2024Queryable)]
pub struct User(pub HackathonUser2024Queryable);

#[derive(Serialize, ToSchema)]
#[schema(title = "VecHackathonUser2024Queryable", value_type = Vec<HackathonUser2024Queryable>, as = Vec<HackathonUser2024Queryable>)]
pub struct VecUser(pub Vec<HackathonUser2024Queryable>);
