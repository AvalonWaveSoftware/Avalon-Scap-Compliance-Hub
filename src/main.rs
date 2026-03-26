use askama::Template;
use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


// NEW: Our dashboard template
#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate;

#[tokio::main]
async fn main() {
    // Initialize logging (required for production tracing & audit logs)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build our web app router (we'll add routes for hosts, scans, etc. later)
let app = Router::new()
        .route("/", get(dashboard_handler))
        .route("/api/hello", get(hello_handler));  // HTMX test route

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 OpenSCAP Compliance Hub listening on {}", addr);

    // NEW: Correct Axum 0.7 way to start the server
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// NEW: Direct rendering — no deprecated crate needed
async fn dashboard_handler() -> Html<String> {
    let template = DashboardTemplate;
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html("<h1 style='color:red'>Template render error</h1>".to_string()),
    }
}

async fn hello_handler() -> &'static str {
    "✅ HTMX works! Server is alive (NIST secure baseline)"
}