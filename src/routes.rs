use crate::handler;
use axum::response::IntoResponse;
use axum::{routing, Router};

/// ## App routes
///
/// The call path is `Router -> Handler`
pub fn with_state<S, AS>(state: AS) -> Router<S>
where
    AS: Clone + Send + Sync + 'static,
{
    Router::new()
        // root
        .route("/", routing::get(root_handler))
        // login
        .route("/login", routing::get(handler::user::login_handler))
        .with_state(state)
}

// ? Test dbp of AppState
async fn root_handler() -> impl IntoResponse {
    tracing::info!("Access to root_handler!");

    "Hello, Askumer-API!"
}
