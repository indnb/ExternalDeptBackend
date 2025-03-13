use crate::diesel::schema::sql_types::HackathonCategory2024;
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

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    FromSqlRow,
    AsExpression,
    Deserialize,
    Serialize,
    utoipa::ToSchema,
)]
#[diesel(sql_type = crate::diesel::schema::sql_types::HackathonCategory2024)]
pub enum HackathonCategory2024Enum {
    Software,
    IoT,
    Gamedev,
    Blockchain,
}
impl ToSql<HackathonCategory2024, Pg> for HackathonCategory2024Enum {
    fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
        match *self {
            HackathonCategory2024Enum::Software => out.write_all(b"software")?,
            HackathonCategory2024Enum::IoT => out.write_all(b"iot")?,
            HackathonCategory2024Enum::Gamedev => out.write_all(b"gamedev")?,
            HackathonCategory2024Enum::Blockchain => out.write_all(b"blockchain")?,
        };
        Ok(IsNull::No)
    }
}
impl FromSql<HackathonCategory2024, Pg> for HackathonCategory2024Enum {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let value = std::str::from_utf8(bytes.as_bytes())?;
        match value {
            "software" => Ok(HackathonCategory2024Enum::Software),
            "iot" => Ok(HackathonCategory2024Enum::IoT),
            "gamedev" => Ok(HackathonCategory2024Enum::Gamedev),
            "blockchain" => Ok(HackathonCategory2024Enum::Blockchain),
            _ => {
                info!("Not a valid hackathon category: {value}, default set 'education'");
                Ok(HackathonCategory2024Enum::Software)
            }
        }
    }
}
