use axum::{extract::Json, http::StatusCode, routing::post, Router};
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
    let user_name = match new_user.validate_name() {
        UserName::Valid(name) => name,
        UserName::Invalid(name, reason) => {
            debug!(
                "Sign-in request with invalid username: \"{}\", reason: {}",
                name, reason
            );
            return StatusCode::BAD_REQUEST;
        }
    };

    debug!("Adding new user {}", user_name);
    return StatusCode::NOT_IMPLEMENTED;
}

#[derive(Deserialize)]
struct NewUser {
    name: String,
}

enum UserName {
    Valid(String),
    Invalid(String, &'static str),
}

impl NewUser {
    fn validate_name(&self) -> UserName {
        if self.name.len() > 12 {
            return UserName::Invalid(self.name.clone(), "name too long");
        }
        return UserName::Valid(self.name.clone());
    }
}
