use std::env;

pub trait AuthorizationProvider {
    fn get_authorization(&self) -> String;
}

pub struct EnvironmentAuthorizationProvider;

impl AuthorizationProvider for EnvironmentAuthorizationProvider {
    fn get_authorization(&self) -> String {
        env::var("N3RGY__APIKEY").expect("Missing required API key. Please set the environment variable 'N3RGY__APIKEY' to your In Home Display (IHD) MAC address to avoid this error.")
    }
}
