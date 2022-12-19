use super::*;
use mockito;
use mockito::mock;
use std::env;

#[tokio::test]
async fn can_get_licenses() {
    env::set_var("GO_PKG_OVERRIDE_URL", &mockito::server_url());
    let go_pkg_resp = r#"
        <a 
        href="/gorm.io/driver/mysql?tab=licenses" 
        data-test-id="UnitHeader-license"
        aria-label="Go to Licenses" 
        data-gtmc="header link"
        >MIT</a>
    "#;
    let m = mock("GET", "/test")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body(go_pkg_resp)
        .create();
    let dep: Vec<String> = vec!["test".to_string()];
    let resp = get_licenses(dep).await;
    m.assert();
    match resp {
        Ok(deps_licenses) => assert_eq!(deps_licenses[0].1, "MIT".to_string()),
        Err(e) => {
            println!("Error: {}", e);
            assert!(false);
        }
    }
}
