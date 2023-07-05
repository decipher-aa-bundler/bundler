pub mod errors;
pub mod handler;
pub mod models;
pub mod service;
pub mod types;

use crate::rpc::types::BundlerClient;

use crate::config::Config;
use actix_web::{web, App, HttpServer};
use log::info;
use mempool::{Mempool, MempoolService};

pub async fn new_server(config_path: String) {
    info!("starting server");

    Config::set(config_path);
    let config = Config::get();
    let chain_id = config.chain_id;

    let mempool = Mempool::new(chain_id);

    HttpServer::new(move || {
        App::new().service(new_service()).app_data(
            new_app_data(&config, Box::new(mempool.clone())).unwrap_or_else(|e| panic!("{}", e)),
        )
    })
    .bind(("0.0.0.0", 8000))
    .unwrap()
    .run()
    .await
    .unwrap()
}

fn new_app_data(
    config: &Config,
    mempool: Box<dyn MempoolService>,
) -> Result<web::Data<BundlerClient>, String> {
    Ok(web::Data::new(BundlerClient::new(config, mempool)?))
}

fn new_service() -> actix_web::Scope {
    web::scope("/api/v1")
        .service(web::scope("user-ops").service(handler::user_ops::send_user_operation))
        .service(web::scope("user-ops-gas").service(handler::user_ops::estimate_user_ops_gas))
}
