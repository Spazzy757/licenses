use select::document::Document;
use select::predicate::Name;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
type AvailableDependency = String;

// The path to the go file
const GOMOD: &str = "./go.mod";

pub fn read_dependencies() -> Option<Vec<std::string::String>> {
    if !Path::new(GOMOD).exists() {
        return None;
    }
    let f = File::open(GOMOD).unwrap();
    let reader = BufReader::new(f);
    let mut block = false;
    let deps: Vec<AvailableDependency> = reader
        .lines()
        .filter_map(|v| match v {
            Ok(line) => {
                let segs: Vec<&str> = line.split(' ').collect();
                if segs.len() == 1 {
                    if segs[0] == ")" {
                        block = false;
                    }
                    return None;
                }
                if block {
                    let nested_dep: Vec<_> = segs.iter().filter(|v| !v.is_empty()).collect();
                    let slice = nested_dep[0].strip_prefix("\t");
                    if slice.is_none() {
                        return None;
                    }
                    return Some(slice.unwrap().to_string());
                }
                if segs[0] == "require" {
                    if segs[1] == "(" {
                        block = true;
                        return None;
                    }
                    let slice = &segs[1..][0].strip_prefix("\t");
                    if slice.is_none() {
                        return None;
                    }
                    return Some(slice.unwrap().to_string());
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

pub async fn get_licenses(deps: Vec<std::string::String>) -> Result<(), Box<dyn Error>> {
    for l in deps {
        let pkg_url = format!("https://pkg.go.dev/{}", l);
        let res = reqwest::get(pkg_url).await?.text().await?;
        let mut license = "unknown".to_string();
        Document::from(res.as_str())
            .find(Name("a"))
            .filter_map(|n| {
                //TODO figure out a better way to get the License from the page
                if !n.attr("data-test-id").is_none()
                    && n.attr("data-test-id").unwrap() == "UnitHeader-license"
                {
                    Some(n.text())
                } else {
                    None
                }
            })
            .for_each(|x| license = x);
        println!("{} {}", l, license);
    }
    Ok(())
}
