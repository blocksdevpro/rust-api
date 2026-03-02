pub mod todos;

use crate::state::AppState;
use axum::Router;

use std::sync::{Arc, Mutex};

pub type SharedState = Arc<Mutex<AppState>>;

pub fn create_router(state: SharedState) -> Router {
    Router::new().nest("/api/todos", todos::create_todos_router(state.clone()))
}
