use crate::server::Server;
mod api;
mod data;
mod diesel;
mod error;
mod server;
mod tests;
mod utils;

#[tokio::main]
async fn main() {
    Server::run().await;
}
