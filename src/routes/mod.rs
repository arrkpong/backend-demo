pub mod user_routes;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    user_routes::configure(cfg);
}
