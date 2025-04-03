use crate::diesel::models::hackathon_2025::user::HackathonUser2025Insertable;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[schema(title = "HackathonUser2025Insertable", value_type = HackathonUser2025Insertable, as = HackathonUser2025Insertable)]
pub struct User(pub HackathonUser2025Insertable);

#[derive(Deserialize, ToSchema)]
#[schema(title = "VecHackathonUser2025Insertable", value_type = Vec<HackathonUser2025Insertable>, as = Vec<HackathonUser2025Insertable>)]
pub struct VecUser(#[allow(dead_code)] pub Vec<HackathonUser2025Insertable>);
