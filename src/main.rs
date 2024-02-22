use std::fmt;

use axum::{
    extract::Json,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use tracing::{debug, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/sign-in", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(Json(new_user): Json<NewUser>) -> StatusCode {
    debug!("Adding new user {}", new_user);
    return StatusCode::NOT_IMPLEMENTED;
}

#[derive(Deserialize)]
struct NewUser {
    username: String,
}

impl fmt::Display for NewUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.username)
    }
}
