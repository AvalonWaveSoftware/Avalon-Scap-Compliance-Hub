use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize logging (required for production tracing & audit logs)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build our web app router (we'll add routes for hosts, scans, etc. later)
    let app = Router::new().route("/", get(root_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 OpenSCAP Compliance Hub listening on {}", addr);

    // NEW: Correct Axum 0.7 way to start the server
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// Simple handler — returns plain text (we'll switch to Askama HTML + HTMX later)
async fn root_handler() -> &'static str {
    "✅ Hello from OpenSCAP Compliance Hub! (NIST Planning phase skeleton ready)"
}