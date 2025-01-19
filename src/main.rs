use crate::server::Server;
mod api;
mod diesel;
mod dto;
mod error;
mod middleware;
mod models;
mod server;
mod swagger;
#[cfg(test)]
mod tests;
mod utils;

#[tokio::main]
async fn main() {
    Server::run().await;
}
