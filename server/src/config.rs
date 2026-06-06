use std::env;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub database_url: String,
}

#[derive(Clone, Debug)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expires_in_seconds: u64,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            server: ServerConfig {
                host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("PORT").unwrap_or_else(|_| "3000".to_string()),
            },
            database: DatabaseConfig {
                database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                    "postgres://nonewhite_user:nonewhite_password@localhost:5432/nonewhite_site"
                        .to_string()
                }),
            },
            auth: AuthConfig {
                jwt_secret: env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "change-me-in-local-development".to_string()),
                jwt_expires_in_seconds: env::var("JWT_EXPIRES_IN_SECONDS")
                    .ok()
                    .and_then(|value| value.parse::<u64>().ok())
                    .unwrap_or(604_800),
            },
        }
    }
}

pub fn server_address() -> String {
    AppConfig::from_env().server.address()
}
