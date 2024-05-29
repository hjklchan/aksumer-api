use axum::Router;
use config::{Env, ENV};
use dotenvy::dotenv;
use sqlx::{MySql, Pool};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod error;
mod extractor;
mod handler;
mod routes;
mod utils;

/// AppState
#[derive(Clone)]
pub struct AppState {
    // Database connection pool with MySQL
    dbp: Pool<MySql>,
    cfg: &'static Env
}

#[tokio::main]
async fn main() {
    // initialize logger tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // initialize database pool
    let dbp = db::init().await;
    tracing::debug!("database connection successful");

    // initialize AppState
    let app_state: AppState = AppState { dbp, cfg: &ENV };

    // make app
    let app = Router::new()
        // merged routes
        .merge(routes::new(app_state))
        .layer(TraceLayer::new_for_http());

    // make tcp listener
    let tcp_listener = TcpListener::bind(&ENV.server.0).await.unwrap();
    tracing::debug!("listening on {}", tcp_listener.local_addr().unwrap());

    // run it
    axum::serve(tcp_listener, app).await.unwrap();
}
