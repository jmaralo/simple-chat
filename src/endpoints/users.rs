use std::sync::Arc;

use axum::{extract::State, http::StatusCode};

use crate::AppState;

pub async fn get(State(state): State<Arc<AppState>>) -> StatusCode {
    return StatusCode::NOT_IMPLEMENTED;
}
