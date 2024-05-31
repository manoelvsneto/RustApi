use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database_url: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let server = ServerConfig {
            host: env::var("SERVER_HOST")?,
            port: env::var("SERVER_PORT")?.parse()?,
        };
        let database_url = env::var("DATABASE_URL")?;

        Ok(Self { server, database_url })
    }
}
