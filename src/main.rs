use clap::Parser;
use log::{error, warn};

mod go;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Remote bool to determine if stored in a git repo
    #[arg(short, long, default_value_t = false)]
    remote: bool,

    /// Config path or URL to config file
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() {
    let _args = Args::parse();

    let deps = go::read_dependencies();
    if deps.is_none() {
        warn!("no dependencies found, are you in the correct directory?")
    }
    let result = go::get_licenses(deps.unwrap()).await;
    match result {
        Ok(_) => (),
        Err(error) => error!("{}", error),
    }
}
