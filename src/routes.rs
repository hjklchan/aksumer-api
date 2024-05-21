use crate::utils::jwt::Claims;
use crate::{handler, AppState};
use axum::response::IntoResponse;
use axum::{routing, Router};

/// ## App routes *with_state*
///
/// The call path is `Router -> Handler`
pub fn with_state<S>(state: AppState) -> Router<S> {
    Router::new()
        // root
        .route("/", routing::get(root_handler))
        // login
        .route("/login", routing::post(handler::users::login_handler))
        // register
        .route("/register", routing::post(handler::users::create_handler))
        // auth required route
        .route("/auth_required", routing::get(auth_required_handler))
        // handle api not found
        .fallback(handler::not_found_handler)
        // state
        .with_state(state)
}

async fn root_handler() -> impl IntoResponse {
    tracing::info!("Access to root_handler!");

    "Hello, Askumer-API!"
}

// ! Remember to delete below handler
/// TEST auth required
async fn auth_required_handler(claims: Claims) -> impl IntoResponse {
    println!("get user payload: {:#?}", claims.payload);
}
