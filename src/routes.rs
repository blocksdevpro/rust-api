use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;

use crate::state::AppState;
use crate::{models::TodoJson, state};

use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    title: String,
}

pub type SharedState = Arc<Mutex<AppState>>;

async fn create_todo_handler(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<TodoJson>, StatusCode> {
    let mut state = state.lock().unwrap();
    match state.create_todo(&payload.title) {
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_todos_handler(State(state): State<SharedState>) -> Json<Vec<TodoJson>> {
    let state = state.lock().unwrap();
    let todos = state.get_todos().into_iter().cloned().collect();
    Json(todos)
}

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/todos", get(get_todos_handler).post(create_todo_handler))
        .with_state(state)
}
