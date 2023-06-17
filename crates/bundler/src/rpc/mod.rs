pub mod errors;
pub mod handler;
pub mod models;
pub mod service;
pub mod types;

use crate::rpc::types::BundlerClient;

use actix_web::{web, App, HttpServer};
use log::info;
use mempool::{Mempool, MempoolService};

pub async fn new_server() {
    info!("starting server");

    // TODO: chain id to lazy static
    let chain_id = 1;
    let mempool = Mempool::new(chain_id);

    HttpServer::new(move || {
        App::new()
            .service(new_service())
            .app_data(new_app_data(Box::new(mempool.clone())).unwrap_or_else(|e| panic!("{}", e)))
    })
    .bind(("0.0.0.0", 8000))
    .unwrap()
    .run()
    .await
    .unwrap()
}

fn new_app_data(mempool: Box<dyn MempoolService>) -> Result<web::Data<BundlerClient>, String> {
    Ok(web::Data::new(BundlerClient::new("", "", mempool)?))
}

fn new_service() -> actix_web::Scope {
    web::scope("/api/v1")
        .service(web::scope("user-ops").service(handler::user_ops::estimate_user_ops_gas))
}
