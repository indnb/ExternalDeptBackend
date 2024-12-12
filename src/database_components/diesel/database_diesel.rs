use crate::utils::env_configuration::EnvConfiguration;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_db_pool() -> DbPool {
    let main_database_url = format!(
        "postgres://{}:{}@{}:{}",
        EnvConfiguration::get().database_user.clone(),
        EnvConfiguration::get().database_password.clone(),
        EnvConfiguration::get().database_host.clone(),
        EnvConfiguration::get().database_port.clone()
    );
    let manager = ConnectionManager::<PgConnection>::new(main_database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
