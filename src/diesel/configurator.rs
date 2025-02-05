use super::prelude::DbState;
use crate::error::api_error::ApiError;
use crate::utils::constants::diesel::MIGRATIONS;
use crate::utils::env_configuration::EnvConfiguration;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sql_types::Text;
use diesel::RunQueryDsl;
use diesel_migrations::MigrationHarness;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn configuration_database() -> DbPool {
    let database_url = format!(
        "postgres://{}:{}@{}:{}",
        EnvConfiguration::get().database_user,
        EnvConfiguration::get().database_password,
        EnvConfiguration::get().database_host,
        EnvConfiguration::get().database_port,
    );

    let database_name = &EnvConfiguration::get().database_name;

    let server_manager =
        ConnectionManager::<PgConnection>::new(format!("{}/postgres", database_url));
    let server_pool = Pool::builder()
        .build(server_manager)
        .unwrap_or_else(|err| panic!("Error creating server connection pool: {}", err));

    let mut connection = server_pool
        .get()
        .expect("Failed to get a connection to the PostgreSQL server");

    let exists = diesel::sql_query("SELECT 1 FROM pg_database WHERE datname = $1")
        .bind::<Text, _>(database_name)
        .execute(&mut connection)
        .map(|rows| rows > 0)
        .unwrap_or(false);

    if !exists {
        println!(
            "Database '{}' does not exist. Creating it now...",
            database_name
        );

        let create_db_query = format!("CREATE DATABASE \"{}\";", database_name);
        diesel::sql_query(create_db_query)
            .execute(&mut connection)
            .unwrap_or_else(|err| panic!("Error creating database '{}': {}", database_name, err));

        println!("Database '{}' created successfully!", database_name);
    } else {
        println!("Database '{}' already exists. Proceeding.", database_name);
    }

    let target_url = format!("{}/{}", database_url, database_name);
    let target_manager = ConnectionManager::<PgConnection>::new(target_url);
    let db_pool = Pool::builder()
        .build(target_manager)
        .unwrap_or_else(|err| panic!("Error creating database connection pool: {}", err));

    let mut target_connection = db_pool
        .get()
        .expect("Failed to get a connection to the target database");

    target_connection
        .run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|err| panic!("Error running migrations: {}", err));

    println!("Pool created successfully!");

    db_pool
}

pub fn get_connection(
    db_pool: &DbState,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, ApiError> {
    db_pool
        .get()
        .map_err(|err| ApiError::FailedDatabaseConnection(err.to_string()))
}
