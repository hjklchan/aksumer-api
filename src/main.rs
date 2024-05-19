use axum::{extract::MatchedPath, http::Request, response::IntoResponse, routing, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", routing::get(root_handler))
        .layer(TraceLayer::new_for_http());

    let tcp_listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();

    axum::serve(tcp_listener, app).await.unwrap();
}

async fn root_handler() -> impl IntoResponse {
    tracing::info!("Access to root_handler!");

    "Hello, Askumer-API!"
}
