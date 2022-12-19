use super::*;
#[test]
fn can_load_yaml_file() {
    //Setup yaml file
    let c = Config {
        allowed_licenses: vec!["MIT".to_string()],
        whitelisted_dependencies: vec!["test".to_string()],
    };
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("test_config.yaml")
        .expect("Couldn't open file");
    serde_yaml::to_writer(f, &c).unwrap();
    //load yaml file
    let mut cfg = Config::default();
    cfg = cfg.load_config("test_config.yaml".to_string());
    println!("{:#?}", cfg);
    assert!(cfg.allowed_licenses == vec!["MIT"]);
    assert!(cfg.whitelisted_dependencies == vec!["test"]);
    //destroy yaml file
    std::fs::remove_file("test_config.yaml").unwrap();
}
#[test]
fn can_load_defaults() {
    let cfg = Config::default();
    assert!(cfg.allowed_licenses.is_empty());
    assert!(cfg.whitelisted_dependencies.is_empty());
}
