use actix_web::{App, HttpServer};
use env_logger::Env;
use crate::rpc::service::new_service;

pub async fn new_server() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(new_service())
    })
        .bind(("0.0.0.0", 8000))
        .unwrap()
        .run()
        .await
        .unwrap()
}
