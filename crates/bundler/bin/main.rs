use bundler::rpc::new_server;

#[actix_web::main]
async fn main() {
    new_server().await
}