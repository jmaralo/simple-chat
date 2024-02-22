use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Router,
};
use tracing::info;

use crate::{
    endpoints::{create_user, users},
    user::UserName,
};

mod endpoints;
mod user;
struct AppState {
    user_list: Mutex<HashMap<UserName, String>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_state = Arc::new(AppState {
        user_list: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/users", get(users::get))
        .route("/new-user", post(create_user::post))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
