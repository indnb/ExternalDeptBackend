use crate::diesel::schema;
use crate::diesel::schema::sql_types::UserRole;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub phone: String,
    pub role: Option<UserRole>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
