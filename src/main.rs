use axum::{routing::{get, post}, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod handlers;
use crate::handlers::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres:///oscap_hub".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres — check peer auth and user oscap-hub");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migration failed");

    tracing::info!("✅ PostgreSQL connected via peer auth + migrations applied");

    let app = Router::new()
        .route("/", get(dashboard_handler))
        .route("/api/dashboard/orgs", get(orgs_handler))
        .route("/api/dashboard/groups", get(groups_handler))
        .route("/api/dashboard/hosts", get(hosts_handler))
        .route("/api/dashboard/reports", get(reports_handler))
        .route("/api/dashboard/new-org-form", get(new_org_form_handler))
        .route("/api/dashboard/create-org", post(create_org_handler))
        .route("/api/dashboard/new-group-form", get(new_group_form_handler))
        .route("/api/dashboard/create-group", post(create_group_handler))
        .route("/api/dashboard/new-host-form", get(new_host_form_handler))
        .route("/api/dashboard/create-host", post(create_host_handler))
        .route("/api/dashboard/org/:org_id", get(org_detail_handler))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 OpenSCAP Compliance Hub listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}