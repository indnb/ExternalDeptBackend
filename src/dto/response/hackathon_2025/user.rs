use crate::diesel::models::hackathon_2025::user::HackathonUser2025Queryable;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[schema(title = "HackathonUser2025Queryable", value_type = HackathonUser2025Queryable, as = HackathonUser2025Queryable)]
pub struct User(pub HackathonUser2025Queryable);

#[derive(Serialize, ToSchema)]
#[schema(title = "VecHackathonUser2025Queryable", value_type = Vec<HackathonUser2025Queryable>, as = Vec<HackathonUser2025Queryable>)]
pub struct VecUser(pub Vec<HackathonUser2025Queryable>);
