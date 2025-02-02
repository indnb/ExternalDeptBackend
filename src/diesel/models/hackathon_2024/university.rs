use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Queryable, Serialize, ToSchema, Clone)]
#[diesel(table_name = crate::diesel::schema::hackathon_university_2024)]
pub struct HackathonUniversity2024Queryable {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "Innovation University")]
    pub name: String,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub created_at: Option<NaiveDateTime>,
    #[schema(example = "2025-01-19T15:06:19.027744")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Deserialize, Serialize, ToSchema)]
#[diesel(table_name = crate::diesel::schema::hackathon_university_2024)]
pub struct HackathonUniversity2024Insertable {
    #[schema(example = "Innovation University")]
    pub name: String,
}
