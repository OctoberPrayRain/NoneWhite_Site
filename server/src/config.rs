use std::env;

pub fn server_address() -> String {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    format!("{host}:{port}")
}
