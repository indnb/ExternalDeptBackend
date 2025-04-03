use super::prelude::*;
use crate::diesel::models::hackathon_2025::university::HackathonUniversity2024Insertable;
use crate::diesel::schema::hackathon_university_2025;
use crate::error::api_error::ApiError;
use crate::utils::constants::diesel::MIGRATIONS;
use crate::utils::env_configuration::EnvConfiguration;
use csv::ReaderBuilder;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::RunQueryDsl;
use diesel_migrations::MigrationHarness;
use log::{info, warn};
use std::fs::File;
use std::io::BufReader;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

const CSV_UNIVERSITY: &str = "mock_db/university.csv";

pub fn configuration_database() -> DbPool {
    let database_url = format!(
        "postgresql://{}:{}@{}:{}",
        EnvConfiguration::get().database_user,
        EnvConfiguration::get().database_password,
        EnvConfiguration::get().database_host,
        EnvConfiguration::get().database_port,
    );

    let database_name = &EnvConfiguration::get().database_name;

    let server_manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    let server_pool = Pool::builder()
        .build(server_manager)
        .unwrap_or_else(|err| panic!("Error creating server connection pool: {}", err));

    let mut connection = server_pool
        .get()
        .expect("Failed to get a connection to the PostgreSQL server");

    let create_db_query = format!("CREATE DATABASE \"{}\";", database_name);
    let query = diesel::sql_query(create_db_query).execute(&mut connection);

    match query {
        Ok(_) => {
            info!("Database {} created successfully!", database_name);
        }
        Err(err) => {
            warn!("Error creating database: {}", err);
        }
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

    load_csv(&db_pool);

    info!("Pool created successfully!");

    db_pool
}

fn load_csv(pool: &DbPool) {
    let count_current_university = hackathon_university_2025::table
        .count()
        .get_result::<i64>(&mut get_connection(pool).unwrap())
        .unwrap();

    if count_current_university > 0 {
        return;
    }

    let file = File::open(CSV_UNIVERSITY).expect("Cannot open file");
    let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(file));

    let mut rows = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let new_row = HackathonUniversity2024Insertable {
            name: record.get(0).unwrap().to_string(),
            name_eng: record.get(1).unwrap().to_string(),
        };

        rows.push(new_row);
    }

    diesel::insert_into(hackathon_university_2025::table)
        .values(&rows)
        .execute(&mut get_connection(pool).unwrap())
        .unwrap();
}

pub fn get_connection(
    db_pool: &DbPool,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, ApiError> {
    db_pool
        .get()
        .map_err(|err| ApiError::FailedDatabaseConnection(err.to_string()))
}
