# CF Services

[![crates.io](https://img.shields.io/crates/v/cf-services.svg)](https://crates.io/crates/cf-services)
[![Rust](https://github.com/Piszmog/cf-services/actions/workflows/rust.yml/badge.svg)](https://github.com/Piszmog/cf-services/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This library is aimed at removing the boilerplate code and let developers just worry about using actually connecting to
the services they have bounded to their app.

## Retrieving VCAP_SERVICES

Simply use `cf_services::get_services_from_env()`.

```rust
use cf_services::get_services_from_env;

fn main() {
    let services = get_services_from_env().unwrap();
    let service = services.get("serviceA").unwrap();
    // Use information about service A to perform actions (such as creating an OAuth2 Client)
}
```

## Retrieving Credentials of a Service

Call `cf_services::get_service_credentials(..)` by passing the `VCAP_SERVICES` marshalled JSON and the name of the
service to retrieve the credentials for. If `VCAP_SERVICES` is guaranteed to be an environment variable
use `cf_services::get_service_cred_from_env(..)`
instead.

```rust
use cf_services::{get_services_from_env, get_service_credentials, get_service_cred_from_env};

fn main() {
    let services = get_services_from_env().unwrap();
    let creds = get_service_credentials(services, "serviceB").unwrap();
    // Use credentials...

    // Retrieve the JSON from the environment
    let creds = get_service_cred_from_env("serviceB").unwrap();
    // Use credentials...
}
```