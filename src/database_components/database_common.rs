use crate::utils::env_configuration::EnvConfiguration;
use eyre::Result;
use native_tls::TlsConnector;
use postgres_native_tls::{MakeTlsConnector, TlsStream};
use tokio_postgres::{Client, Connection, Socket};

#[allow(dead_code)]
pub async fn create() -> Result<()> {
    let name_db = EnvConfiguration::get().database_name.as_str();

    let (client, connection) = get_connection(&format!(
        "host={} user={} password={} port={}",
        EnvConfiguration::get().database_host.as_str(),
        EnvConfiguration::get().database_user.as_str(),
        EnvConfiguration::get().database_password.as_str(),
        EnvConfiguration::get().database_port.to_string().as_str(),
    ))
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error in admin connection: {}", e);
        }
    });

    let row = client
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)",
            &[&name_db],
        )
        .await?;

    let db_exists: bool = row.get(0);

    if !db_exists {
        if let Err(e) = client
            .execute(format!("CREATE DATABASE {}", name_db).as_str(), &[])
            .await
        {
            eprintln!("Failed to create database_components '{}': {}", name_db, e);
        }
        println!("CREATE DATABASE {}", name_db);
    } else {
        println!("Database '{}' already exists.", name_db);
    }

    Ok(())
}
#[allow(dead_code)]
pub async fn set_up(_client: &Client) -> Result<()> {
    Ok(())
}

#[allow(dead_code)]
pub fn spawn_tokio(connection: Connection<Socket, TlsStream<Socket>>) {
    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!(
                "Error in connection to {}: {}",
                EnvConfiguration::get().database_name,
                error
            );
        }
    });
}
#[allow(dead_code)]
pub async fn get_connection(
    config: &str,
) -> Result<(Client, Connection<Socket, TlsStream<Socket>>)> {
    let connector = TlsConnector::builder()
        .danger_accept_invalid_hostnames(true) // Warning: Disable for production!
        .build()?;
    let connector = MakeTlsConnector::new(connector);
    let (client, connection) = tokio_postgres::connect(config, connector).await?;
    Ok((client, connection))
}
