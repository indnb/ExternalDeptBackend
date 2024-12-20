use diesel::sql_types::Text;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use serde::{Serialize, Deserialize};
use diesel::FromSqlRow;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, AsExpression, FromSqlRow, Eq, PartialEq)]
#[diesel(sql_type = Text)]
pub enum UserRole {
    Admin,
    User,
}