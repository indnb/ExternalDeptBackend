use crate::error::api_error::ApiError;
use crate::utils::constants::database::MIGRATIONS;
use crate::utils::env_configuration::EnvConfiguration;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use diesel_migrations::MigrationHarness;
use rocket::State;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_db_pool() -> DbPool {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        EnvConfiguration::get().database_user,
        EnvConfiguration::get().database_password,
        EnvConfiguration::get().database_host,
        EnvConfiguration::get().database_port,
        EnvConfiguration::get().database_name,
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|err| panic!("Error creating DB pool: {}", err));

    let mut conn = pool
        .get()
        .expect("Failed to get a DB connection for migrations");
    conn.run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|err| panic!("Error running migrations: {}", err));
    println!("Migrations ran successfully.");

    pool
}
pub fn get_connection(
    db_pool: &State<DbPool>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, ApiError> {
    db_pool.get().map_err(|_| ApiError::InternalServerError)
}
