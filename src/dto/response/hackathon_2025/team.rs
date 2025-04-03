use crate::diesel::models::hackathon_2025::{
    category::HackathonCategory2025Enum, team::HackathonTeam2025Queryable,
    user::HackathonUser2025Queryable,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[schema(title = "HackathonTeam2025Queryable", value_type = HackathonTeam2025Queryable, as = HackathonTeam2025Queryable)]
pub struct Team(pub HackathonTeam2025Queryable);

#[derive(Serialize, ToSchema)]
#[schema(title = "VecHackathonTeam2025Queryable", value_type = Vec<HackathonTeam2025Queryable>, as = Vec<HackathonTeam2025Queryable>)]
pub struct VecTeam(pub Vec<HackathonTeam2025Queryable>);

#[derive(Serialize, ToSchema)]
pub struct FullTeam {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "Skalse")]
    pub name: String,
    #[schema(example = "Education")]
    pub category: HackathonCategory2025Enum,
    #[schema(example = "1")]
    pub count_members: i32,
    #[schema(example = "2023-01-19T15:06:19.027744")]
    pub created_at: Option<NaiveDateTime>,
    #[schema(example = "2023-01-19T15:06:19.027744")]
    pub updated_at: Option<NaiveDateTime>,
    pub captain: HackathonUser2025Queryable,
    pub members: Vec<HackathonUser2025Queryable>,
}
