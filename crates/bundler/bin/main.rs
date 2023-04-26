use bundler::rpc::server::new_server;

#[actix_web::main]
async fn main() {
    new_server().await
}