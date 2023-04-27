pub mod user_ops;

use actix_web::{web, Scope};

use crate::rpc::handler::user_ops::estimate_user_ops_gas;

pub fn new_service() -> Scope {
    web::scope("/api/v1").service(web::scope("user-ops").service(estimate_user_ops_gas))
    // .service(web::scope("estimate").service( ))
}
