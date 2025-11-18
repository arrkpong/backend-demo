mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use std::collections::HashSet;
use std::sync::Mutex;

use actix_web::{App, HttpServer, web};
use config::AppConfig;
use db::establish_connection;
use routes::configure as configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig::from_env();
    let db_connection = establish_connection(&app_config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    let shared_connection = web::Data::new(db_connection);
    let shared_config = web::Data::new(app_config.clone());
    let revoked_tokens = web::Data::new(Mutex::new(HashSet::<String>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(shared_connection.clone())
            .app_data(shared_config.clone())
            .app_data(revoked_tokens.clone())
            .configure(configure_routes)
    })
    .bind(&app_config.bind_address)?
    .run()
    .await
}
