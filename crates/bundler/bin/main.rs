use bundler::rpc::new_server;
use env_logger::Env;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    new_server().await
}
