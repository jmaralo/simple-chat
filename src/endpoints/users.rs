use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use tracing::debug;

use crate::state::{user::UserName, AppState};

pub async fn get(State(state): State<Arc<AppState>>) -> (StatusCode, Json<UserList>) {
    let users = match state.user_list.lock() {
        Ok(guard) => guard,
        Err(error) => {
            debug!("user_list mutex poisoned: {}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UserList { users: Vec::new() }),
            );
        }
    };

    let user_list = users
        .keys()
        .map(|name_ref| name_ref.clone())
        .collect::<Vec<UserName>>();

    debug!("Sending list of users");
    return (StatusCode::OK, Json(UserList { users: user_list }));
}

#[derive(Serialize)]
pub struct UserList {
    users: Vec<UserName>,
}
