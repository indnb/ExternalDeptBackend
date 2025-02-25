use crate::api_query::pong::pong;
use crate::api_query::user::create_user;
use crate::diesel::database_diesel::{init_db_pool, DbPool};
use crate::utils::env_configuration::EnvConfiguration;
use log::LevelFilter;
use rocket::figment::Figment;
use rocket::{routes, Config};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::net::IpAddr;

pub struct Server;

impl Server {
    pub async fn run() {
        Server::configure_logging();

        let config = Server::get_server_config().expect("Failed to configure Rocket server");
        let cors = Server::configure_cors();
        let db_pool = init_db_pool();
        Server::build_rocket(db_pool, config, cors).await;
    }

    fn configure_logging() {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Info)
            .init();
    }

    fn get_server_config() -> Result<Config, rocket::figment::Error> {
        let (address, port) = Server::parse_address_port();

        Figment::from(Config::default())
            .merge(("address", address.to_string()))
            .merge(("port", port))
            .extract()
    }

    fn parse_address_port() -> (IpAddr, u16) {
        (
            "0.0.0.0".parse().unwrap(),
            EnvConfiguration::get().server_port,
        )
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
            .mount(
                "/api",
                routes![
                    pong,        // GET api/pong
                    create_user  // POST api/user
                ],
            )
            .launch()
            .await
            .expect("Failed to launch Rocket server");
    }
}
