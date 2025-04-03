use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::diesel::schema::hackathon_team_captain_2025)]
pub struct HackathonTeamCaptain2025Insertable {
    pub team_id: i32,
    pub captain_id: i32,
}

#[derive(Serialize, Queryable, Debug, ToSchema)]
#[diesel(table_name = crate::diesel::schema::hackathon_team_captain_2025)]
pub struct HackathonTeamCaptain2025Queryable {
    #[schema(example = "1")]
    pub team_id: i32,
    #[schema(example = "1")]
    pub captain_id: i32,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub created_at: Option<NaiveDateTime>,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub updated_at: Option<NaiveDateTime>,
}
