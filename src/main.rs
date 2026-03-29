use askama::Template;
use axum::{response::Html, routing::get, Router, extract::State};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Dashboard template (unchanged)
#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // NEW: Secure peer-auth connection (no password, localhost socket only)
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres:///oscap_hub".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres — check peer auth and user oscap-hub");

    // Run migrations automatically (NIST CM-2)
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migration failed");

    tracing::info!("✅ PostgreSQL connected via peer auth + migrations applied");

    let app = Router::new()
        .route("/", get(dashboard_handler))
        .route("/api/hello", get(hello_handler))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 OpenSCAP Compliance Hub listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn dashboard_handler(State(pool): State<PgPool>) -> Html<String> {
    let _org_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM organizations")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let template = DashboardTemplate;
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html("<h1 style='color:red'>Template error</h1>".to_string()),
    }
}

async fn hello_handler() -> &'static str {
    "✅ HTMX + secure DB works! (NIST Development phase)"
}