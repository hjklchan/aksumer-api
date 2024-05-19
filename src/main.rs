use axum::{response::IntoResponse, routing, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;

mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // initialize logger tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // initialize database pool
    db::init().await;
    tracing::debug!("database connection successful");

    // make app
    let app = Router::new()
        .route("/", routing::get(root_handler))
        .layer(TraceLayer::new_for_http());

    // make tcp listener
    let tcp_listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    tracing::debug!("listening on {}", tcp_listener.local_addr().unwrap());

    // run it
    axum::serve(tcp_listener, app).await.unwrap();
}

async fn root_handler() -> impl IntoResponse {
    tracing::info!("Access to root_handler!");

    "Hello, Askumer-API!"
}
