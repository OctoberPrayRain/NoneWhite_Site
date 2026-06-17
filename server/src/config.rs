use std::{env, fmt, path::PathBuf};

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
pub struct UploadConfig {
    pub upload_dir: PathBuf,
    pub public_base_url: String,
    pub max_avatar_size_bytes: usize,
    pub max_image_size_bytes: usize,
    pub max_resource_size_bytes: usize,
}

#[derive(Clone)]
pub struct OpenListConfig {
    pub token: String,
}

impl fmt::Debug for OpenListConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenListConfig")
            .field("token", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub upload: UploadConfig,
    pub openlist: OpenListConfig,
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
            upload: UploadConfig {
                upload_dir: resolve_upload_dir(
                    env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string()),
                ),
                public_base_url: normalize_public_base_url(
                    env::var("UPLOAD_PUBLIC_BASE_URL").unwrap_or_else(|_| "/uploads".to_string()),
                ),
                max_avatar_size_bytes: env::var("MAX_AVATAR_SIZE_BYTES")
                    .ok()
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(2 * 1024 * 1024),
                max_image_size_bytes: env::var("MAX_IMAGE_SIZE_BYTES")
                    .ok()
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(5 * 1024 * 1024),
                max_resource_size_bytes: env::var("MAX_RESOURCE_SIZE_BYTES")
                    .ok()
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(50 * 1024 * 1024),
            },
            openlist: OpenListConfig {
                token: env::var("OPENLIST_TOKEN").unwrap_or_default(),
            },
        }
    }
}

fn resolve_upload_dir(value: String) -> PathBuf {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        return path;
    }

    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
}

fn normalize_public_base_url(value: String) -> String {
    let trimmed = value.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return "/uploads".to_string();
    }

    if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

pub fn server_address() -> String {
    AppConfig::from_env().server.address()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openlist_config_debug_redacts_token() {
        let config = OpenListConfig {
            token: "secret-openlist-token".to_string(),
        };
        let output = format!("{config:?}");

        assert!(output.contains("<redacted>"));
        assert!(!output.contains("secret-openlist-token"));
    }
}
