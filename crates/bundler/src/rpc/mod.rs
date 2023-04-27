pub mod errors;
pub mod handler;
pub mod service;
pub mod types;

use crate::rpc::service::new_service;
use actix_web::{App, HttpServer, web};
use env_logger::Env;
use ethers::providers::{Http, Provider};
use crate::BundlerClient;
use crate::ethereum::EthClient;
use crate::rpc::types::EthClient;

pub async fn new_server() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(new_service())
            .app_data(new_app_data()?)
    })
        .bind(("0.0.0.0", 8000))
        .unwrap()
        .run()
        .await
        .unwrap()
}

fn new_app_data() -> Result<web::Data<BundlerClient>, ()> {
    Ok(web::Data::new(BundlerClient::new(EthClient::new()?)))
}
