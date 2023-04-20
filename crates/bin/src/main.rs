use bundler::rpc::server::new_server;

#[tokio::main]
async fn main() {
    new_server().await;
}
