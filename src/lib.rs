#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # cf-services
//!
//! The `cf-services` crate provides an easy way to retrieve information about services bounded to
//! an application in Cloud Foundry.
//!
//! It retrieves and parses the `cf_services::VCAP_SERVICES` environment variable into a struct
//! for easier consumption.
//!
//! ## Retrieving Services
//!
//! To retrieve all the services, simply use `cf_services::get_services_from_env`.
//!
//! ## Service Credential
//!
//! To retrieve a service's credential information, either use
//! `cf_services::get_service_cred_from_env` or the convenience function
//! `cf_services::get_service_credentials`.

use std::{env, fmt};
use std::collections::HashMap;
use std::fmt::Formatter;

use serde::Deserialize;

/// The environment variable key that contains all the bounded services to the application.
pub const VCAP_SERVICES: &str = "VCAP_SERVICES";

/// The high level service information for a service bounded to an application. Multiple services of
/// the same type can be bounded to an application (e.g. multiple Config Servers).
#[derive(Deserialize, Debug)]
pub struct Service {
    /// The name of the service.
    #[serde(default)]
    pub name: String,
    /// The instance name of the service.
    #[serde(default)]
    pub instance_name: String,
    /// The name the service is bounded as.
    #[serde(default)]
    pub binding_name: String,
    /// The credentials of the service.
    pub credentials: Credentials,
    /// The label associated with the service.
    #[serde(default)]
    pub label: String,
}

/// The credentials information for authenticating with the service.
#[derive(Deserialize, Debug, Clone)]
pub struct Credentials {
    /// The URI of the service.
    #[serde(default)]
    pub uri: String,
    /// The JDBC URI of the service.
    #[serde(rename(deserialize = "jdbcUrl"))]
    #[serde(default)]
    pub jdbc_url: String,
    /// The API URI of the service.
    #[serde(rename(deserialize = "http_api_uri"))]
    #[serde(default)]
    pub api_uri: String,
    /// Th license key for the service.
    #[serde(rename(deserialize = "licenseKey"))]
    #[serde(default)]
    pub license_key: String,
    /// The Client Secret for generating a token via OAuth.
    #[serde(default)]
    pub client_secret: String,
    #[serde(default)]
    /// The Client ID for generating a token via OAuth.
    pub client_id: String,
    #[serde(default)]
    /// The Access Token URI for generating a token via OAuth.
    pub access_token_uri: String,
    /// The hostname of the service.
    #[serde(default)]
    pub hostname: String,
    /// The username to authenticate with the service.
    #[serde(default)]
    pub username: String,
    /// The password of the username.
    #[serde(default)]
    pub password: String,
    /// The Port of the service.
    #[serde(default)]
    pub port: i16,
    /// The name of the credentials.
    #[serde(default)]
    pub name: String,
}

/// Retrieves the credential information of the specified service.
pub fn get_service_cred_from_env(service_name: String) -> Result<Vec<Credentials>, CFError> {
    get_services_from_env()
        .and_then(|services| get_service_credentials(services, service_name))
}

/// Retrieves all service information.
pub fn get_services_from_env() -> Result<HashMap<String, Vec<Service>>, CFError> {
    env::var(VCAP_SERVICES)
        .map_err(|_| CFError::EnvNotSet)
        .and_then(|val| serde_json::from_str(&val).map_err(|_| CFError::MalformedJSON))
}

/// Retrieves the credential information from the provided services that match the specified service
/// name.
pub fn get_service_credentials(services: HashMap<String, Vec<Service>>, service_name: String) -> Result<Vec<Credentials>, CFError> {
    match services.get(&service_name) {
        Some(services) => Ok(services.iter().map(|service| service.credentials.clone()).collect()),
        None => Err(CFError::ServiceNotPresent(service_name))
    }
}

/// Enumeration of the different errors that can occur.
#[derive(PartialEq, Debug)]
pub enum CFError {
    /// Error when the environment variable is not set.
    EnvNotSet,
    /// Error then the environment variable JSON is malformed.
    MalformedJSON,
    /// Error when a service is not present.
    ServiceNotPresent(String),
}

impl fmt::Display for CFError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            CFError::EnvNotSet => write!(f, "environment variable {:?} is not set", VCAP_SERVICES),
            CFError::MalformedJSON => write!(f, "environment variable {:?} is malformed", VCAP_SERVICES),
            CFError::ServiceNotPresent(ref s) => write!(f, "service {:?} is not bounded to the application", s)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;

    use crate::{CFError, Credentials, get_service_cred_from_env, get_service_credentials, get_services_from_env, Service, VCAP_SERVICES};

    #[test]
    fn test_get_service_cred_from_env() {
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

    #[test]
    fn test_get_services_from_env() {
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
        let services = get_services_from_env().unwrap();
        let service_a = services.get("serviceA").unwrap();
        assert_eq!(1, service_a.len());
        let service_details = service_a.get(0).unwrap();
        assert_eq!("service_a", service_details.name);
        assert_eq!("example_uri", service_details.credentials.uri);
        assert_eq!(8080, service_details.credentials.port);
        env::remove_var(VCAP_SERVICES);
    }

    #[test]
    fn test_get_services_from_env_not_set() {
        env::remove_var(VCAP_SERVICES);
        let err = get_services_from_env().err().unwrap();
        assert_eq!(CFError::EnvNotSet, err);
    }

    #[test]
    fn test_get_services_from_env_malformed_json() {
        let json = r#"{
      "serviceA": [
        {
          "name":"service_a",
          "credentials": {
            "uri": "example_uri"
          }
        }
      ]"#;
        env::set_var(VCAP_SERVICES, json);
        let err = get_services_from_env().err().unwrap();
        assert_eq!(CFError::MalformedJSON, err);
        env::remove_var(VCAP_SERVICES);
    }

    #[test]
    fn test_get_service_credentials() {
        let mut services = HashMap::new();
        let mut service_a: Vec<Service> = Vec::new();
        let service = Service {
            name: "service_a".to_string(),
            instance_name: "".to_string(),
            binding_name: "".to_string(),
            credentials: Credentials {
                uri: "example_uri".to_string(),
                jdbc_url: "".to_string(),
                api_uri: "".to_string(),
                license_key: "".to_string(),
                client_secret: "".to_string(),
                client_id: "".to_string(),
                access_token_uri: "".to_string(),
                hostname: "".to_string(),
                username: "".to_string(),
                password: "".to_string(),
                port: 0,
                name: "".to_string(),
            },
            label: "".to_string(),
        };
        service_a.push(service);
        services.insert("serviceA".to_string(), service_a);
        let creds = get_service_credentials(services, "serviceA".to_string()).unwrap();
        assert_eq!(1, creds.len());
        let cred = creds.get(0).unwrap();
        assert_eq!("example_uri", cred.uri);
    }

    #[test]
    fn test_get_service_credentials_not_found() {
        let mut services = HashMap::new();
        let service_a: Vec<Service> = Vec::new();
        services.insert("serviceA".to_string(), service_a);
        let err = get_service_credentials(services, "serviceB".to_string()).err().unwrap();
        assert_eq!(CFError::ServiceNotPresent("serviceB".to_string()), err)
    }
}
