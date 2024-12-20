use crate::diesel::models::users_data::users_role::UserRoleEnum;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Deserialize;

#[derive(Debug, Queryable)]
#[diesel(table_name = crate::diesel::schema::users)]
pub struct User {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub first_name: String,
    #[allow(dead_code)]
    pub last_name: String,
    #[allow(dead_code)]
    pub password_hash: String,
    #[allow(dead_code)]
    pub phone: String,
    #[allow(dead_code)]
    pub role: UserRoleEnum,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = crate::diesel::schema::users)]
pub struct InsertableUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password_hash: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub role: UserRoleEnum,
}
