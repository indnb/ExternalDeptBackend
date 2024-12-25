/* WILL UNCOMMENT WHEN IN SCHEMA.RS EXISTS USER_ROLE!!!
use crate::diesel::schema::sql_types::UserRole;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::{serialize, FromSqlRow};
use log::info;
use serde::{Deserialize, Serialize};
use serialize::IsNull;
use std::fmt::Debug;
use std::io::Write;
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromSqlRow, AsExpression, Deserialize, Serialize)]
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
impl FromSql<UserRole, Pg> for UserRoleEnum {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let value = std::str::from_utf8(bytes.as_bytes())?;
        match value {
            "Admin" => Ok(UserRoleEnum::Admin),
            "User" => Ok(UserRoleEnum::User),
            _ => {
                info!("Not a valid user role: {}, default set 'User'", value);
                Ok(UserRoleEnum::User)
            }
        }
    }
}
*/
