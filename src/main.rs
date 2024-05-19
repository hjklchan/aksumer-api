use axum::{response::IntoResponse, routing, Router};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", routing::get(root_handler));

    let tcp_listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();

    axum::serve(tcp_listener, app).await.unwrap();
}

async fn root_handler() -> impl IntoResponse {
    "Hello, Askumer-API!"
}
