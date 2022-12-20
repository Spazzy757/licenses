use log::{error, info};
use std::error::Error;
use std::process;

type Unapproved = Vec<(String, String)>;

// check_licenses is where the magic happens
// we go through each dependency file and find the dependencies and their licenses
// we then validate these are allowed against our config
// TODO add support for:
// - rust
// - javascript
// - python
// - ruby
// - java
pub async fn check_licenses(cfg: super::config::Config) -> Result<Unapproved, Box<dyn Error>> {
    info!("starting dependencies check, this might take a while");
    // For now we only support Go
    let deps = super::go::read_dependencies();
    if deps.is_none() {
        error!("no dependencies found, are you in the correct directory?");
        process::exit(1);
    }
    let result = super::go::get_licenses(deps.unwrap()).await;
    let mut unapproved: Unapproved = vec![];
    //TODO I dont like doing this here
    //we should handle this in the "get_license"
    //functionality and only return unapproved dependencies
    match result {
        Ok(licenses) => {
            'outer: for license in licenses {
                for w in &cfg.whitelisted_dependencies {
                    if w.contains(&license.0) {
                        continue 'outer;
                    }
                }
                for l in &cfg.allowed_licenses {
                    if license.1.contains(l) {
                        continue 'outer;
                    }
                }
                unapproved.push(license);
            }
        }
        Err(error) => error!("{}", error),
    }
    Ok(unapproved)
}
