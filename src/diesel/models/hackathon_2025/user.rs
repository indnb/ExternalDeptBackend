use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Queryable, Clone, Serialize, ToSchema)]
#[diesel(table_name = crate::diesel::schema::hackathon_user_2025)]
pub struct HackathonUser2025Queryable {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "Skalse")]
    pub first_name: String,
    #[schema(example = "Batya")]
    pub last_name: String,
    #[schema(example = "skalse_456")]
    pub nickname_tg: Option<String>,
    #[schema(example = "1234567890")]
    pub phone: Option<String>,
    #[schema(example = "1")]
    pub university_id: i32,
    #[schema(example = "1")]
    pub team_id: i32,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub created_at: Option<NaiveDateTime>,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Default, Deserialize, Serialize, ToSchema)]
#[diesel(table_name = crate::diesel::schema::hackathon_user_2025)]
pub struct HackathonUser2025Insertable {
    #[schema(example = "Skalse")]
    pub first_name: String,
    #[schema(example = "Batya")]
    pub last_name: String,
    #[schema(example = "skalse_456")]
    pub nickname_tg: Option<String>,
    #[schema(example = "+380123456789")]
    pub phone: Option<String>,
    #[schema(example = "1")]
    pub university_id: i32,
    #[schema(example = "1")]
    pub team_id: i32,
}
