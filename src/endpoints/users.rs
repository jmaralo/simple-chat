use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use tracing::debug;

use crate::state::{
    user::{Name, User},
    AppState,
};

pub async fn list(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Option<UserList>>) {
    let Ok(user_list) = state.users() else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    };

    debug!("Sending list of users");
    return (StatusCode::OK, Json(Some(UserList { users: user_list })));
}

#[derive(Serialize)]
pub struct UserList {
    users: Vec<Name>,
}

pub async fn get(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> (StatusCode, Json<Option<UserInfo>>) {
    let user_name = match Name::try_from(name.clone()) {
        Ok(name) => name,
        Err(reason) => {
            debug!("Invalid user name \"{}\", reason: {}", name, reason);
            return (StatusCode::BAD_REQUEST, Json(None));
        }
    };

    let Ok(possible_user) = state.get_user(&user_name) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    };

    let Some(user) = possible_user else {
        return (StatusCode::NOT_FOUND, Json(None));
    };

    return (StatusCode::OK, Json(Some(UserInfo::from(user))));
}

#[derive(Serialize)]
pub struct UserInfo {
    name: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            name: user.name().to_string(),
        }
    }
}
