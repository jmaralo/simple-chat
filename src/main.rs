use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router,
};
use serde::Deserialize;
use tracing::{debug, info, warn};

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
        .route("/new-user", post(create_user))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<NewUser>,
) -> StatusCode {
    let user_name = match new_user.validate_name() {
        Ok(name) => name,
        Err(reason) => {
            debug!(
                "Sign-in request with invalid username: \"{}\", reason: {}",
                new_user.name, reason
            );
            return StatusCode::BAD_REQUEST;
        }
    };

    let mut user_list = match state.user_list.lock() {
        Ok(guard) => guard,
        Err(error) => {
            debug!("user_list mutex poisoned: {}", error);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if (*user_list).contains_key(&user_name) {
        info!("Tried to create already existing user: \"{}\"", user_name);
        return StatusCode::CONFLICT;
    }

    info!("Adding new user: \"{}\"", user_name);
    user_list.insert(user_name, new_user.password);
    return StatusCode::CREATED;
}

#[derive(Deserialize)]
struct NewUser {
    name: String,
    password: String,
}

impl NewUser {
    fn validate_name(&self) -> Result<UserName, &'static str> {
        if self.name.len() > 12 {
            return Err("name too long");
        }
        return Ok(UserName(self.name.clone()));
    }
}

#[derive(Hash, Eq, PartialEq)]
struct UserName(String);

impl Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
