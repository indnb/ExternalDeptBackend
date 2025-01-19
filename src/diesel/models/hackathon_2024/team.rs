use crate::diesel::models::hackathon_2024::category::HackathonCategory2024Enum;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, utoipa::ToSchema)]
#[diesel(table_name = crate::diesel::schema::hackathon_team_2024)]
pub struct HackathonTeam2024Queryable {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "Team 1")]
    pub name: String,
    #[schema(example = "Education")]
    pub category: HackathonCategory2024Enum,
    #[schema(example = "$2b$12$9pK8ha/nYGLwRWG53UK/EuOlGYJjS/aXPCPKxdX3o7UfiuQrtFDiq")]
    pub password_registration: String,
    #[schema(example = "2")]
    pub count_members: i32,
    #[schema(example = "skalse_456")]
    pub nickname_tg: String,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub created_at: Option<NaiveDateTime>,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Deserialize, Serialize, utoipa::ToSchema)]
#[diesel(table_name = crate::diesel::schema::hackathon_team_2024)]
pub struct HackathonTeam2024Insertable {
    #[schema(example = "Team 1")]
    pub name: String,
    #[schema(example = "Education")]
    pub category: HackathonCategory2024Enum,
    #[schema(example = "real_password123!AAA")]
    pub password_registration: String,
    #[schema(example = "skalse_456")]
    pub nickname_tg: String,
}
