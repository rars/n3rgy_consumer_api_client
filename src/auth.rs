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

pub struct StaticAuthorizationProvider {
    api_key: String,
}

impl StaticAuthorizationProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl AuthorizationProvider for StaticAuthorizationProvider {
    fn get_authorization(&self) -> String {
        self.api_key.clone()
    }
}
