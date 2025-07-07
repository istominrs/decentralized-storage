use crate::api::health::health_handler;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health_handler);
}
