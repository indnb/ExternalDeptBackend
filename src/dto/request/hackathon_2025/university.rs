use crate::diesel::models::hackathon_2025::university::HackathonUniversity2025Insertable;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[schema(title = "HackathonUniversity2024Insertable", value_type = HackathonUniversity2025Insertable, as = HackathonUniversity2025rtable)]
pub struct University(pub HackathonUniversity2025Insertable);

#[derive(Deserialize, ToSchema)]
#[schema(title = "VecHackathonUniversity2024Insertable", value_type = Vec<HackathonUniversity2025Insertable>, as = Vec<HackathonUniversity2025Insertable>)]
pub struct VecUniversity(pub Vec<HackathonUniversity2025Insertable>);
