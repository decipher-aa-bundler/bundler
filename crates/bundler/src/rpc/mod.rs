pub mod errors;
pub mod handler;
pub mod service;
pub mod types;

use crate::rpc::service::new_service;
use actix_web::{App, HttpServer};
use env_logger::Env;

pub async fn new_server() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().service(new_service()))
        .bind(("0.0.0.0", 8000))
        .unwrap()
        .run()
        .await
        .unwrap()
}
