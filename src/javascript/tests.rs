use super::*;
use std::fs;

#[test]
fn can_read_dependencies() {
    let package_json = r#"
{
  "dependencies": {
    "lodash": "^4.17.20",
    "zod": "^3.14.4"
  }
}"#
    .to_string();
    fs::write("package.json", package_json).expect("Unable to create file");
    let deps = read_dependencies();
    if deps.is_none() {
        assert!(false, "return empty dependency list");
    }
    let res = deps.unwrap();
    println!("{:#?}", res);
    fs::remove_file("package.json").unwrap();
}

#[tokio::test]
async fn can_get_licenses() {
    let res: Vec<String> = vec!["lodash".to_string()];
    get_licenses(res).await;
}
