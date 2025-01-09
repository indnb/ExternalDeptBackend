use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize)]
#[diesel(table_name = crate::diesel::schema::hackathon_university_2024)]
pub struct HackathonUniversity2024Queryable {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = crate::diesel::schema::hackathon_university_2024)]
pub struct HackathonUniversity2024Insertable {
    pub name: String,
}
