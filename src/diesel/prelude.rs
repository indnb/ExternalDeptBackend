pub use crate::diesel::configurator::get_connection;
pub use crate::diesel::configurator::DbPool;
pub use crate::error::api_error::ApiError;
pub use diesel::ExpressionMethods;
pub use diesel::QueryDsl;
pub use diesel::RunQueryDsl;
pub use rocket::State;
pub type DbState = State<DbPool>;
