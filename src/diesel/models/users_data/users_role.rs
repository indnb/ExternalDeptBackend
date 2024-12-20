use crate::diesel::schema::sql_types::UserRole;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::{serialize, FromSqlRow};
use serde::Deserialize;
use serialize::IsNull;
use std::fmt::Debug;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromSqlRow, AsExpression, Deserialize)]
#[diesel(sql_type = crate::diesel::schema::sql_types::UserRole)]
pub enum UserRoleEnum {
    Admin,
    User,
}
impl ToSql<UserRole, Pg> for UserRoleEnum {
    fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
        match *self {
            UserRoleEnum::Admin => out.write_all(b"Admin")?,
            UserRoleEnum::User => out.write_all(b"User")?,
        };
        Ok(IsNull::No)
    }
}
