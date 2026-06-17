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
    pub base_url: String,
    pub token: String,
    pub resource_upload_dir: String,
}

impl fmt::Debug for OpenListConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenListConfig")
            .field("base_url", &self.base_url)
            .field("token", &"<redacted>")
            .field("resource_upload_dir", &self.resource_upload_dir)
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

impl OpenListConfig {
    fn from_env() -> Self {
        Self {
            base_url: env::var("OPENLIST_BASE_URL").unwrap_or_default(),
            token: env::var("OPENLIST_TOKEN").unwrap_or_default(),
            resource_upload_dir: env::var("OPENLIST_RESOURCE_UPLOAD_DIR").unwrap_or_default(),
        }
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
            openlist: OpenListConfig::from_env(),
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
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn openlist_config_debug_redacts_token() {
        let config = OpenListConfig {
            base_url: "https://openlist.example".to_string(),
            token: "secret-openlist-token".to_string(),
            resource_upload_dir: "openlist:/GoogleDrive/uploads".to_string(),
        };
        let output = format!("{config:?}");

        assert!(output.contains("<redacted>"));
        assert!(!output.contains("secret-openlist-token"));
    }

    #[test]
    fn openlist_config_loads_base_url_token_and_resource_upload_dir_from_env() {
        let _guard = ENV_LOCK.lock().expect("env lock should not be poisoned");
        const BASE_URL: &str = "OPENLIST_BASE_URL";
        const TOKEN: &str = "OPENLIST_TOKEN";
        const UPLOAD_DIR: &str = "OPENLIST_RESOURCE_UPLOAD_DIR";
        let previous_base_url = env::var(BASE_URL).ok();
        let previous_token = env::var(TOKEN).ok();
        let previous_upload_dir = env::var(UPLOAD_DIR).ok();

        env::set_var(BASE_URL, "https://openlist.example");
        env::set_var(TOKEN, "upload-token");
        env::set_var(UPLOAD_DIR, "openlist:/GoogleDrive/uploads");
        let config = OpenListConfig::from_env();
        restore_env(BASE_URL, previous_base_url);
        restore_env(TOKEN, previous_token);
        restore_env(UPLOAD_DIR, previous_upload_dir);

        assert_eq!(config.base_url, "https://openlist.example");
        assert_eq!(config.token, "upload-token");
        assert_eq!(config.resource_upload_dir, "openlist:/GoogleDrive/uploads");
    }

    #[test]
    fn openlist_config_defaults_to_blank_fields_without_failing_startup() {
        let _guard = ENV_LOCK.lock().expect("env lock should not be poisoned");
        const BASE_URL: &str = "OPENLIST_BASE_URL";
        const TOKEN: &str = "OPENLIST_TOKEN";
        const UPLOAD_DIR: &str = "OPENLIST_RESOURCE_UPLOAD_DIR";
        let previous_base_url = env::var(BASE_URL).ok();
        let previous_token = env::var(TOKEN).ok();
        let previous_upload_dir = env::var(UPLOAD_DIR).ok();

        env::remove_var(BASE_URL);
        env::remove_var(TOKEN);
        env::remove_var(UPLOAD_DIR);
        let config = OpenListConfig::from_env();
        restore_env(BASE_URL, previous_base_url);
        restore_env(TOKEN, previous_token);
        restore_env(UPLOAD_DIR, previous_upload_dir);

        assert!(config.base_url.is_empty());
        assert!(config.token.is_empty());
        assert!(config.resource_upload_dir.is_empty());
    }

    fn restore_env(key: &str, value: Option<String>) {
        match value {
            Some(value) => env::set_var(key, value),
            None => env::remove_var(key),
        }
    }
}
