use std::time::Duration;

#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
    pub brand_name: String,
    pub brand_color: String,
    pub cors_origins: Vec<String>,
    pub rate_qps: u32,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let port = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
        let brand_name = std::env::var("BRAND_NAME").unwrap_or_else(|_| "{{WRAPPER_NAME}}".to_string());
        let brand_color = std::env::var("BRAND_COLOR").unwrap_or_else(|_| "{{THEME_COLOR}}".to_string());
        let cors = std::env::var("CORS_ORIGINS").unwrap_or_else(|_| "*".to_string());
        let cors_origins = cors.split(',').map(|s| s.trim().to_string()).collect();
        let rate_qps = std::env::var("RATE_QPS").ok().and_then(|s| s.parse().ok()).unwrap_or(20);
        Self { port, brand_name, brand_color, cors_origins, rate_qps }
    }
    pub fn rate_window(&self) -> Duration { Duration::from_secs(1) }
}
