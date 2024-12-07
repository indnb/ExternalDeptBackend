extern crate rocket;
use crate::utils::env_configuration::EnvConfiguration;
use log::LevelFilter;
use reqwest::Client;
use rocket::figment::Figment;
use rocket::{routes, Config};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use sqlx::PgPool;
use std::net::IpAddr;
#[allow(dead_code)]
pub async fn set_up_rocket(db_pool: PgPool) {
    configure_logging();

    let config = get_server_config().expect("Failed to configure Rocket server");
    let cors = configure_cors();
    let client = Client::new();

    build_rocket(db_pool, config, cors, client).await;
}

#[allow(dead_code)]
fn configure_logging() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
}
#[allow(dead_code)]
fn get_server_config() -> Result<Config, rocket::figment::Error> {
    let (address, port) = parse_address_port();

    Figment::from(Config::default())
        .merge(("address", address.to_string()))
        .merge(("port", port))
        .extract()
}

#[allow(dead_code)]
fn parse_address_port() -> (IpAddr, u16) {
    let port = EnvConfiguration::get().server_port;

    ("0.0.0.0".parse().unwrap(), port)
}
#[allow(dead_code)]
fn configure_cors() -> Cors {
    let exact = &[&format!("https://{}", EnvConfiguration::get().main_url)];
    CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(exact),
        allowed_methods: vec!["GET", "POST", "PUT", "DELETE"]
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error while building CORS")
}
#[allow(dead_code)]
async fn build_rocket(db_pool: PgPool, config: Config, cors: Cors, client: Client) {
    rocket::custom(config)
        .attach(cors)
        .attach(rocket::shield::Shield::default())
        .manage(db_pool)
        .manage(client)
        .mount("/api", routes![])
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}
