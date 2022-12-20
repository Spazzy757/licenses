use clap::Parser;
use env_logger;
use log::{error, info};
use std::process;

mod config;
mod go;
mod license_check;

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
    env_logger::init();
    let _args = Args::parse();

    info!("loading config from {}", _args.config);
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
    info!("config loaded successfully");

    match license_check::check_licenses(cfg).await {
        Ok(u) => {
            if u.len() == 0 {
                info!("all packages approved for use");
                process::exit(0);
            }
            for pkg in u {
                error!("package {} with license {} not allowed", pkg.0, pkg.1);
            }
            process::exit(1);
        }
        Err(e) => {
            error!("error: {}", e);
            process::exit(1);
        }
    };
}
