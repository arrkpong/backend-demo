use actix_web::web;

use crate::handlers::{
    auth_handler::{login, logout, register},
    user_handler::{index, profile},
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(profile);
    cfg.service(logout);
    cfg.service(login);
    cfg.service(register);
}
