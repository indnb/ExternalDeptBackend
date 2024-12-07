use crate::utils::env_configuration::EnvConfiguration;
use eyre::Result;
use sqlx::{postgres::PgPoolOptions, Connection, Executor, PgConnection, PgPool};

#[allow(dead_code)]
pub async fn init_db_pool() -> Result<PgPool> {
    let database_name = EnvConfiguration::get().database_name.clone();
    println!("{database_name}");
    let main_database_url = format!(
        "postgres://{}:{}@{}:{}",
        EnvConfiguration::get().database_user.clone(),
        EnvConfiguration::get().database_password.clone(),
        EnvConfiguration::get().database_host.clone(),
        EnvConfiguration::get().database_port.clone()
    );
    let database_url = format!("{}/{}", main_database_url, database_name);

    let mut main_conn = PgConnection::connect(&main_database_url).await?;

    let db_check_query = format!(
        "SELECT 1 FROM pg_database WHERE datname = '{}';",
        database_name
    );
    let db_exists: Option<(i32,)> = sqlx::query_as(&db_check_query)
        .fetch_optional(&mut main_conn)
        .await?;

    if db_exists.is_none() {
        sqlx::query(&format!("CREATE DATABASE \"{}\";", database_name))
            .execute(&mut main_conn)
            .await
            .expect("Failed to create database");
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    pool.execute(
        r#" CREATE TABLE IF NOT EXISTS test {
            id SERIAL PRIMARY KEY,
            test VARCHAR(255)
        }         
        "#,
    )
    .await?;

    Ok(pool)
}
