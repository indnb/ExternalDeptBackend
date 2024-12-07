use once_cell::sync::OnceCell;
use std::env;

static CONFIG: OnceCell<EnvConfiguration> = OnceCell::new();
pub struct EnvConfiguration {
    pub database_name: String,
    pub database_user: String,
    pub database_password: String,
    pub database_host: String,
    pub database_port: u16,
    pub main_url: String,
    pub server_port: u16,
}

impl EnvConfiguration {
    pub fn get() -> &'static EnvConfiguration {
        dotenv::dotenv().ok();
        CONFIG.get_or_init(|| EnvConfiguration {
            database_name: EnvConfiguration::unwrap_env("DATABASE_NAME", None),
            database_user: EnvConfiguration::unwrap_env("DATABASE_USER", None),
            database_password: EnvConfiguration::unwrap_env("DATABASE_PASSWORD", None),
            database_host: EnvConfiguration::unwrap_env(
                "DATABASE_HOST",
                Some("localhost".to_string()),
            ),
            database_port: EnvConfiguration::unwrap_env("DATABASE_PORT", Some(5432.to_string()))
                .parse()
                .expect("Invalid DATABASE_PORT"),
            main_url: EnvConfiguration::unwrap_env(
                "DATABASE_PASSWORD",
                Some("http://localhost:3000".to_string()),
            ),
            server_port: EnvConfiguration::unwrap_env("SERVER_PORT", Some(8080.to_string()))
                .parse()
                .expect("Invalid SERVER_PORT"),
        })
    }
    fn unwrap_env(name: &str, default: Option<String>) -> String {
        env::var(name).unwrap_or_else(|_| match default {
            None => panic!("Error loading environment variable: {}", name),
            Some(some) => some,
        })
    }
}
