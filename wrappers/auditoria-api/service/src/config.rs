use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub service_port: u16,
    pub public_url_base: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { service_port: 8080, public_url_base: "http://localhost:8080".into() }
    }
}
