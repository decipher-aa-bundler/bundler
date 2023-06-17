mod cli;

use crate::cli::Command;
use bundler::rpc::new_server;
use clap::{CommandFactory, Parser};
use env_logger::Env;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    match Command::try_parse() {
        Ok(cmd) => new_server(cmd.config_path).await,
        Err(_) => {
            Command::command().print_help().unwrap();
        }
    }
}
