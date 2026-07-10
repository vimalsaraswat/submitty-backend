use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub db_url: String,
    pub server_url: String,
}

impl Config {
    pub fn init() -> Config {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
        let server_url = format!("{host}:{port}");

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        Config { db_url, server_url }
    }
}
