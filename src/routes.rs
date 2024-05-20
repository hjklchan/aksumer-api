use crate::{handler, AppState};
use axum::response::IntoResponse;
use axum::{routing, Router};

/// ## App routes
///
/// The call path is `Router -> Handler`
pub fn with_state<S>(state: AppState) -> Router<S> {
    Router::new()
        // root
        .route("/", routing::get(root_handler))
        // login
        .route("/login", routing::post(handler::user::login_handler))
        // register
        .route("/register", routing::post(handler::user::create_handler))
        .with_state(state)
}

// ? Test dbp of AppState
async fn root_handler() -> impl IntoResponse {
    tracing::info!("Access to root_handler!");

    "Hello, Askumer-API!"
}
