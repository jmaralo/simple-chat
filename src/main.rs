use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tracing::info;

use crate::{
    endpoints::{create_user, users},
    state::AppState,
};

mod endpoints;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/users", get(users::list))
        .route("/users/:user-name", get(users::get))
        .route("/new-user", post(create_user::post))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
