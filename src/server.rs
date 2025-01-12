use crate::api;
use crate::diesel::configurator::{configuration_database, DbPool};
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
        let db_pool = configuration_database();
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

    async fn build_rocket(db_pool: DbPool, config: Config, cors: Cors) {
        rocket::custom(config)
            .attach(cors)
            .manage(db_pool)
            .mount(
                "/api",
                routes![
                    // /test/*
                    api::test::get::ping,
                    // /hackathon_2024/user/*
                    api::hackathon_2024::user::post::registration_by_tg,
                    api::hackathon_2024::user::get::all,
                    api::hackathon_2024::user::put::by_id,
                    api::hackathon_2024::user::delete::by_id,
                    // /hackathon_2024/university/*
                    api::hackathon_2024::university::post::create,
                    api::hackathon_2024::university::get::all,
                    api::hackathon_2024::university::get::by_id,
                    api::hackathon_2024::university::put::by_id,
                    api::hackathon_2024::university::delete::by_id,
                    // /hackathon_2024/team/*
                    api::hackathon_2024::team::post::create,
                    api::hackathon_2024::team::get::all,
                    api::hackathon_2024::team::get::by_id,
                    api::hackathon_2024::team::put::by_id,
                    api::hackathon_2024::team::delete::by_id,
                    // /adnmin/
                    api::admin::post::login,
                    api::admin::get::get,
                    // /other/*
                ],
            )
            .launch()
            .await
            .expect("Failed to launch Rocket server");
    }
}
