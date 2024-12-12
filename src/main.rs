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
    set_up_rocket().await;
}
