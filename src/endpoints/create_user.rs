use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::Deserialize;
use tracing::debug;

use crate::state::{
    user::{Name, User},
    AddUserError, AppState,
};

pub async fn post(State(state): State<Arc<AppState>>, Json(new_user): Json<NewUser>) -> StatusCode {
    let user_name = match Name::try_from(new_user.name.clone()) {
        Ok(name) => name,
        Err(reason) => {
            debug!(
                "Invalid user name \"{}\", reason: {}",
                new_user.name, reason
            );
            return StatusCode::BAD_REQUEST;
        }
    };

    let user = User::new(user_name);

    return match state.add_user(user) {
        Ok(()) => StatusCode::CREATED,
        Err(AddUserError::AlreadyExists) => StatusCode::CONFLICT,
        Err(AddUserError::Other) => StatusCode::INTERNAL_SERVER_ERROR,
    };
}

#[derive(Deserialize)]
pub struct NewUser {
    name: String,
}
