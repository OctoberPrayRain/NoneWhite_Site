mod config;
mod response;
mod routes;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::from_path("../.env").ok();
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .merge(routes::api_routes())
        .fallback(routes::not_found);
    let address = config::server_address();
    let listener = TcpListener::bind(&address)
        .await
        .expect("failed to bind backend server address");

    info!("NoneWhite_Site Rust API server is running at http://{address}");
    axum::serve(listener, app)
        .await
        .expect("failed to start backend server");
}
