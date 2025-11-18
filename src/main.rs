mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod state;
mod utils;

use actix_web::{App, HttpServer, web};
use config::AppConfig;
use db::establish_connection;
use routes::configure as configure_routes;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig::from_env();
    let db_connection = establish_connection(&app_config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    let shared_state = web::Data::new(AppState::new(db_connection, app_config.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .configure(configure_routes)
    })
    .bind(&app_config.bind_address)?
    .run()
    .await
}
