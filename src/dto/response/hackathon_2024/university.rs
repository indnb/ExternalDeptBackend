use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Queryable;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[schema(title = "HackathonUniversity2024Queryable", value_type = HackathonUniversity2024Queryable, as = HackathonUniversity2024Queryable)]
pub struct University(pub HackathonUniversity2024Queryable);

#[derive(Serialize, ToSchema)]
#[schema(title = "VecHackathonUniversity2024Queryable", value_type = Vec<HackathonUniversity2024Queryable>, as = Vec<HackathonUniversity2024Queryable>)]
pub struct VecUniversity(pub Vec<HackathonUniversity2024Queryable>);
