use std::env;

use cf_services::{get_service_cred_from_env, VCAP_SERVICES};

#[test]
fn integration_test() {
    let json = r#"{
      "serviceA": [
        {
          "name":"service_a",
          "credentials": {
            "uri": "example_uri",
            "port": 8080
          }
        }
      ]
    }"#;
    env::set_var(VCAP_SERVICES, json);
    let creds = get_service_cred_from_env("serviceA".to_string()).unwrap();
    assert_eq!(1, creds.len());
    let cred = creds.get(0).unwrap();
    assert_eq!("example_uri", cred.uri);
    assert_eq!(8080, cred.port);
    env::remove_var(VCAP_SERVICES);
}
