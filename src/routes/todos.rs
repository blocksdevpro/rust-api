use crate::models::TodoJson;
use crate::routes::SharedState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    title: Option<String>,
    completed: Option<bool>,
}

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

async fn get_todos_handler(
    State(state): State<SharedState>,
) -> Result<Json<Vec<TodoJson>>, StatusCode> {
    let state = state.lock().unwrap();
    let todos = state.get_todos().into_iter().cloned().collect();
    Ok(Json(todos))
}

async fn get_todo_handler(
    State(state): State<SharedState>,
    Path(id): Path<u32>,
) -> Result<Json<TodoJson>, StatusCode> {
    let state = state.lock().unwrap();
    match state.get_todo(id) {
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_todos_handler(
    State(state): State<SharedState>,
    Path(id): Path<u32>,
) -> Result<Json<TodoJson>, StatusCode> {
    let mut state = state.lock().unwrap();
    match state.delete_todo(id) {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_todos_handler(
    State(state): State<SharedState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<TodoJson>, StatusCode> {
    let mut state = state.lock().unwrap();
    match state.update_todo(id, payload.title.as_deref(), payload.completed) {
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub fn create_todos_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(get_todos_handler).post(create_todo_handler))
        .route(
            "/:id",
            get(get_todo_handler)
                .delete(delete_todos_handler)
                .patch(update_todos_handler),
        )
        .with_state(state)
}
