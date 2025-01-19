use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Queryable, Serialize, ToSchema)]
#[diesel(table_name = crate::diesel::schema::hackathon_user_2024)]
pub struct HackathonUser2024Queryable {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "Skalse")]
    pub first_name: String,
    #[schema(example = "Batya")]
    pub last_name: String,
    #[schema(example = "skalse_456")]
    pub nickname_tg: String,
    #[schema(example = "1234567890")]
    pub phone: String,
    #[schema(example = "1")]
    pub university_id: Option<i32>,
    #[schema(example = "1")]
    pub team_id: Option<i32>,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub created_at: Option<NaiveDateTime>,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Deserialize, Serialize, ToSchema)]
#[diesel(table_name = crate::diesel::schema::hackathon_user_2024)]
pub struct HackathonUser2024Insertable {
    #[schema(example = "Skalse")]
    pub first_name: String,
    #[schema(example = "Batya")]
    pub last_name: String,
    #[schema(example = "skalse_456")]
    pub nickname_tg: String,
    #[schema(example = "+380123456789")]
    pub phone: String,
    #[schema(example = "1")]
    pub university_id: i32,
    #[schema(example = "1")]
    pub team_id: i32,
}
