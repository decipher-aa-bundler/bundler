pub mod errors;
pub mod handler;
pub mod service;
pub mod types;

use crate::ethereum::EthClient;
use crate::rpc::service::new_service;
use crate::BundlerClient;
use actix_web::{web, App, HttpServer};
use env_logger::Env;

pub async fn new_server() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(new_service())
            .app_data(new_app_data().unwrap_or_else(|e| panic!("{}", e)))
    })
    .bind(("0.0.0.0", 8000))
    .unwrap()
    .run()
    .await
    .unwrap()
}

fn new_app_data() -> Result<web::Data<BundlerClient>, String> {
    Ok(web::Data::new(BundlerClient::new(
        EthClient::new().map_err(|e| e.to_string())?,
    )))
}
