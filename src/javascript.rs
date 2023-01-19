use log::info;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// The path to the go file
const PACKAGEJSON: &str = "./package.json";

type AvailableDependency = String;

type DependencyLicense = (String, String);

//read_dependencies reads the dependency file and creates a vector
//to easier fetch licensing information
//TODO add version information
pub fn read_dependencies() -> Option<Vec<std::string::String>> {
    if !Path::new(PACKAGEJSON).exists() {
        return None;
    }
    let f = File::open(PACKAGEJSON).unwrap();
    let reader = BufReader::new(f);
    let mut block = false;
    let deps: Vec<AvailableDependency> = reader
        .lines()
        .filter_map(|v| match v {
            Ok(line) => {
                let segs: Vec<&str> = line.split(' ').collect();
                if segs[segs.len() - 1] == "}," || segs[segs.len() - 1] == "}" {
                    block = false;
                }
                if block {
                    return Some(segs[segs.len() - 2].replace("\":", "").replace("\"", ""));
                }
                if segs.len() == 4 && segs[segs.len() - 2].contains("dependencies") {
                    block = true;
                }
                return None;
            }
            Err(_) => {
                return None;
            }
        })
        .collect();
    Some(deps)
}

//get_licenses fetches javascript licenses from https://www.npmjs.com/ if none is available
//then it will set the license type to "unknown"
pub async fn get_licenses(
    deps: Vec<std::string::String>,
) -> Result<Vec<DependencyLicense>, Box<dyn Error>> {
    let base_url =
        std::env::var("NPM_OVERRIDE_URL").unwrap_or("https://registry.npmjs.com".to_string());
    let mut deps_licenses: Vec<DependencyLicense> = vec![];
    for l in deps {
        info!("checking license for {}", l);
        let pkg_url = format!("{}/{}", base_url, l);
        let res = reqwest::get(pkg_url).await?.text().await?;
        let mut license = "unknown".to_string();
        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(res.as_str())?;
        if v["license"] != Value::Null {
            license = v["license"].to_string().replace("\"", "");
        }
        println!("{:#?}", license);
        deps_licenses.push((l, license))
    }
    Ok(deps_licenses)
}

#[cfg(test)]
mod tests;
