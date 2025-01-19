use serde::Deserialize;
use utoipa::ToSchema;

use crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Insertable;

#[derive(Deserialize, ToSchema)]
#[schema(title = "HackathonUniversity2024Insertable", value_type = HackathonUniversity2024Insertable, as = HackathonUniversity2024Insertable)]
pub struct University(pub HackathonUniversity2024Insertable);

#[derive(Deserialize, ToSchema)]
#[schema(title = "VecHackathonUniversity2024Insertable", value_type = Vec<HackathonUniversity2024Insertable>, as = Vec<HackathonUniversity2024Insertable>)]
pub struct VecUniversity(pub Vec<HackathonUniversity2024Insertable>);
