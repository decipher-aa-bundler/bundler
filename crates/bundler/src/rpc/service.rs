pub mod user_ops;

use actix_web::{web, Scope};

use crate::rpc::handler::user_ops::request;

pub fn new_service() -> Scope {
    web::scope("/api/v1")
        .service(web::scope("user-ops").service(request))
        // .service(web::scope("estimate").service( ))
}
