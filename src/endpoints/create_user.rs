use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::Deserialize;
use tracing::{debug, info};

use crate::state::{user::UserName, AppState};

pub async fn post(State(state): State<Arc<AppState>>, Json(new_user): Json<NewUser>) -> StatusCode {
    let user_name = match new_user.validate_name() {
        Ok(name) => name,
        Err(reason) => {
            debug!("Invalid username \"{}\", reason: {}", new_user.name, reason);
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
        info!("User \"{}\" already exists", user_name);
        return StatusCode::CONFLICT;
    }

    info!("Adding new user \"{}\"", user_name);
    user_list.insert(user_name, new_user.password);
    return StatusCode::CREATED;
}

#[derive(Deserialize)]
pub struct NewUser {
    name: String,
    password: String,
}

impl NewUser {
    fn validate_name(&self) -> Result<UserName, &'static str> {
        if self.name.len() > 12 {
            return Err("name too long");
        }
        Ok(UserName::from(self.name.clone()))
    }
}
