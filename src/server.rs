use std::net::IpAddr;
use crate::database_components::diesel::database_diesel::{init_db_pool, test_connection, DbPool};
use log::LevelFilter;
use rocket::{routes, Config};
use rocket::figment::Figment;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use crate::api_query::pong::pong;
use crate::utils::env_configuration::EnvConfiguration;

pub async fn set_up_rocket() {
    configure_logging();

    let config = get_server_config().expect("Failed to configure Rocket server");
    let cors = configure_cors();
    let pool = init_db_pool();

    build_rocket(pool, config, cors).await;
}

fn configure_logging() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
}

fn get_server_config() -> Result<Config, rocket::figment::Error> {
    let (address, port) = parse_address_port();

    Figment::from(Config::default())
        .merge(("address", address.to_string()))
        .merge(("port", port))
        .extract()
}

fn parse_address_port() -> (IpAddr, u16) {
    ("0.0.0.0".parse().unwrap(), EnvConfiguration::get().server_port)
}

fn configure_cors() -> Cors {
    let exact = &[&format!("https://{}", "your_main_url.com")];
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

async fn build_rocket(db_pool: DbPool, config: Config, cors: Cors) {
    rocket::custom(config)
        .attach(cors)
        .manage(db_pool)
        .mount("/api", routes![pong])
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}
