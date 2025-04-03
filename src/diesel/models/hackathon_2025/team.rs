use crate::diesel::models::hackathon_2025::category::HackathonCategory2025Enum;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, utoipa::ToSchema)]
#[diesel(table_name = crate::diesel::schema::hackathon_team_2025)]
pub struct HackathonTeam2025Queryable {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "Team 1")]
    pub name: String,
    #[schema(example = "Education")]
    pub category: HackathonCategory2025Enum,
    #[schema(example = "2")]
    pub count_members: i32,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub created_at: Option<NaiveDateTime>,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Deserialize, Serialize, utoipa::ToSchema)]
#[diesel(table_name = crate::diesel::schema::hackathon_team_2025)]
pub struct HackathonTeam2025Insertable {
    #[schema(example = "Team 1")]
    pub name: String,
    #[schema(example = "Education")]
    pub category: HackathonCategory2025Enum,
}
