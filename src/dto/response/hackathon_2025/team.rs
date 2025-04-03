use crate::diesel::models::hackathon_2025::team::HackathonTeam2025Queryable;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[schema(title = "HackathonTeam2025Queryable", value_type = HackathonTeam2025Queryable, as = HackathonTeam2025Queryable)]
pub struct Team(pub HackathonTeam2025Queryable);

#[derive(Serialize, ToSchema)]
#[schema(title = "VecHackathonTeam2025Queryable", value_type = Vec<HackathonTeam2025Queryable>, as = Vec<HackathonTeam2025Queryable>)]
pub struct VecTeam(pub Vec<HackathonTeam2025Queryable>);
