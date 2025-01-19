use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Queryable;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[schema(title = "HackathonTeam2024Queryable", value_type = HackathonTeam2024Queryable, as = HackathonTeam2024Queryable)]
pub struct Team(pub HackathonTeam2024Queryable);

#[derive(Serialize, ToSchema)]
#[schema(title = "VecHackathonTeam2024Queryable", value_type = Vec<HackathonTeam2024Queryable>, as = Vec<HackathonTeam2024Queryable>)]
pub struct VecTeam(pub Vec<HackathonTeam2024Queryable>);
