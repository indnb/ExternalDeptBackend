use crate::api;
use crate::diesel::configurator::{configuration_database, DbPool};
use crate::dto::response::hackathon_2025::university::update_university_cached;
use crate::swagger::ApiDoc;
use crate::utils::env_configuration::EnvConfiguration;
use log::LevelFilter;
use rocket::figment::Figment;
use rocket::{routes, Config, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::net::IpAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct Server;

impl Server {
    pub async fn run() {
        Server::configure_logging();

        let config = Server::get_server_config().expect("Failed to configure Rocket server");
        let cors = Server::configure_cors();
        let db_pool = configuration_database();

        let db_state = rocket::build().manage(db_pool.clone());
        let db_state: Option<&State<DbPool>> = State::get(&db_state);
        update_university_cached(db_state.expect("Failed to configure Rocket server"))
            .await
            .expect("Failed to configure Rocket server");

        Server::build_rocket(db_pool, config, cors).await;
    }

    fn configure_logging() {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Info)
            .init();
    }

    fn get_server_config() -> Result<Config, Box<rocket::figment::Error>> {
        let (address, port) = Server::parse_address_port();

        Figment::from(Config::default())
            .merge(("address", address.to_string()))
            .merge(("port", port))
            .extract()
            .map_err(Box::new)
    }

    fn parse_address_port() -> (IpAddr, u16) {
        (
            "0.0.0.0".parse().unwrap(),
            EnvConfiguration::get().server_port,
        )
    }

    fn configure_cors() -> Cors {
        let exact = &[
            &format!("http://{}", EnvConfiguration::get().main_url),
            &format!("https://{}", EnvConfiguration::get().main_url),
            &format!("http://0.0.0.0:{}", EnvConfiguration::get().server_port),
            &format!("http://127.0.0.1:{}", EnvConfiguration::get().server_port),
        ];
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
                "/",
                SwaggerUi::new("/swagger-ui/<_..>").url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .mount(
                "/api",
                routes![
                    // /test/*
                    api::test::get::ping,
                    // /hackathon_2025/user/*
                    api::hackathon_2025::user::post::create,
                    api::hackathon_2025::user::get::all,
                    api::hackathon_2025::user::put::by_id,
                    api::hackathon_2025::user::delete::by_id,
                    api::hackathon_2025::user::get::by_id,
                    api::hackathon_2025::user::get::by_university,
                    api::hackathon_2025::user::get::by_team,
                    // /hackathon_2025/university/*
                    api::hackathon_2025::university::post::create,
                    api::hackathon_2025::university::post::create_by_vec,
                    api::hackathon_2025::university::get::all,
                    api::hackathon_2025::university::get::by_id,
                    api::hackathon_2025::university::put::by_id,
                    api::hackathon_2025::university::delete::by_id,
                    // /hackathon_2025/team/*
                    api::hackathon_2025::team::post::registration,
                    api::hackathon_2025::team::post::create,
                    api::hackathon_2025::team::get::all,
                    api::hackathon_2025::team::get::by_id,
                    api::hackathon_2025::team::put::by_data,
                    api::hackathon_2025::team::delete::by_id,
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
