use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "bundler")]
pub struct Command {
    pub config_path: String,
}
