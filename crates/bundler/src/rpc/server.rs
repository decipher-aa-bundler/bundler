use actix_web::{App, HttpServer};
use actix_web::dev::Server;
use env_logger::Env;

pub async fn new_server() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new())
        .bind(("0.0.0.0", 8080))
        .unwrap()
        .run()
        .await
        .unwrap()
}
