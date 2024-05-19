use axum::{response::IntoResponse, routing, Router};
use dotenvy::dotenv;
use sqlx::{MySql, Pool};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod error;
mod handler;
mod routes;

/// AppState
#[derive(Clone)]
pub struct AppState {
    // Database connection pool with MySQL
    dbp: Pool<MySql>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // initialize logger tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // initialize database pool
    let dbp = db::init().await;
    tracing::debug!("database connection successful");

    // initialize AppState
    let app_state: AppState = AppState { dbp };

    // make app
    let app = Router::new()
        // merged routes
        .merge(routes::with_state(app_state))
        .layer(TraceLayer::new_for_http());

    // make tcp listener
    let tcp_listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    tracing::debug!("listening on {}", tcp_listener.local_addr().unwrap());

    // run it
    axum::serve(tcp_listener, app).await.unwrap();
}
