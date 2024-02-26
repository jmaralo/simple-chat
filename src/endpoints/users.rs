use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use tracing::debug;

use crate::state::{user::Name, AppState, UsersError};

pub async fn get(State(state): State<Arc<AppState>>) -> (StatusCode, Json<UserList>) {
    let user_list = match state.users() {
        Ok(users) => users,
        Err(UsersError::Other) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UserList { users: Vec::new() }),
            );
        }
    };

    debug!("Sending list of users");
    return (StatusCode::OK, Json(UserList { users: user_list }));
}

#[derive(Serialize)]
pub struct UserList {
    users: Vec<Name>,
}
