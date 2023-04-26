use actix_web::{Scope, web};

pub mod user_ops;

pub fn new_service() -> Scope {
    web::scope("/api/v1")
        .service(web::scope("user-ops").service(user_ops::handle))
    // .service()
}