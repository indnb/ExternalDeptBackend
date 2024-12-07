use crate::database_components::database_sqlx::init_db_pool;
use crate::server::set_up_rocket;
mod api_query;
mod data;
mod database_components;
mod error;
mod server;
mod tests;
mod utils;

#[tokio::main]
async fn main() {
    let db_pool = init_db_pool().await;

    set_up_rocket(db_pool.unwrap()).await;
}
