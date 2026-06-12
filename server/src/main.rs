pub mod config;
pub mod db;
pub mod dto;
pub mod error;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod response;
pub mod routes;
pub mod services;
pub mod state;

use axum::Router;
use std::path::Path;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    load_env_files();
    tracing_subscriber::fmt::init();

    let config = config::AppConfig::from_env();
    let db_pool = db::create_pool(&config.database).expect("failed to create database pool");
    let address = config.server.address();
    let state = state::AppState { config, db_pool };

    let app = Router::new()
        .merge(routes::api_routes())
        .fallback(routes::not_found)
        .with_state(state);
    let listener = TcpListener::bind(&address)
        .await
        .expect("failed to bind backend server address");

    info!("NoneWhite_Site Rust API server is running at http://{address}");
    axum::serve(listener, app)
        .await
        .expect("failed to start backend server");
}

fn load_env_files() {
    let server_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root_dir = server_dir
        .parent()
        .expect("server directory should have project root");

    load_env_files_from_paths(server_dir.join(".env"), root_dir.join(".env"));
}

fn load_env_files_from_paths(server_env_path: impl AsRef<Path>, root_env_path: impl AsRef<Path>) {
    dotenvy::from_path(server_env_path).ok();
    dotenvy::from_path(root_env_path).ok();
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        path::PathBuf,
        sync::Mutex,
        time::{SystemTime, UNIX_EPOCH},
    };

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn load_env_files_prefers_server_env_over_root_env() {
        let _guard = ENV_LOCK.lock().expect("env lock should not be poisoned");
        let key = unique_env_key("NW_SERVER_WINS");
        let temp_dir = unique_temp_dir("server-wins");
        let root_env = temp_dir.join("root.env");
        let server_env = temp_dir.join("server.env");

        fs::write(&root_env, format!("{key}=root\n")).expect("root env should be writable");
        fs::write(&server_env, format!("{key}=server\n")).expect("server env should be writable");
        env::remove_var(&key);

        super::load_env_files_from_paths(&server_env, &root_env);

        assert_eq!(env::var(&key).as_deref(), Ok("server"));
        env::remove_var(&key);
        fs::remove_dir_all(temp_dir).expect("temp env dir should be removable");
    }

    #[test]
    fn load_env_files_keeps_root_env_when_server_env_omits_key() {
        let _guard = ENV_LOCK.lock().expect("env lock should not be poisoned");
        let key = unique_env_key("NW_ROOT_FALLBACK");
        let server_only_key = unique_env_key("NW_SERVER_ONLY");
        let temp_dir = unique_temp_dir("root-fallback");
        let root_env = temp_dir.join("root.env");
        let server_env = temp_dir.join("server.env");

        fs::write(&root_env, format!("{key}=root\n")).expect("root env should be writable");
        fs::write(&server_env, format!("{server_only_key}=value\n"))
            .expect("server env should be writable");
        env::remove_var(&key);
        env::remove_var(&server_only_key);

        super::load_env_files_from_paths(&server_env, &root_env);

        assert_eq!(env::var(&key).as_deref(), Ok("root"));
        env::remove_var(&key);
        env::remove_var(&server_only_key);
        fs::remove_dir_all(temp_dir).expect("temp env dir should be removable");
    }

    fn unique_env_key(prefix: &str) -> String {
        format!("{prefix}_{}", unique_suffix())
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let path = env::temp_dir().join(format!("nonewhite-env-{label}-{}", unique_suffix()));
        fs::create_dir_all(&path).expect("temp env dir should be creatable");
        path
    }

    fn unique_suffix() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos()
    }
}
