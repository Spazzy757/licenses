use clap::Parser;
use log::{error, warn};
use std::process;

mod config;
mod go;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Remote bool to determine if stored in a git repo
    #[arg(short, long, default_value_t = false)]
    remote: bool,

    /// Token string for remote config, will set the header Authorization: Bearer $token
    #[arg(short, long, default_value = "")]
    token: String,

    /// Config path or URL to config file
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() {
    let _args = Args::parse();

    // Load config
    let mut cfg = config::Config::default();
    if _args.remote {
        match cfg.load_remote_config(_args.config, _args.token).await {
            Ok(c) => cfg = c,
            Err(e) => {
                error!("error: {}", e);
                process::exit(1);
            }
        }
    } else {
        cfg = cfg.load_config(_args.config);
    }

    //    let deps = go::read_dep bnencies();
    //    if deps.is_none() {
    //        warn!("no dependencies found, are you in the correct directory?")
    //    }
    //    let result = go::get_licenses(deps.unwrap()).await;
    //    match result {
    //        Ok(_) => (),
    //        Err(error) => error!("{}", error),
    //    }
}
