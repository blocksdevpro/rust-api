use std::sync::{Arc, Mutex};

use axum::{Router, routing::get};

use crate::models::TodoJson;

mod models;
mod routes;
mod state;

#[allow(unused)]
fn print_todo(todo: &TodoJson) {
    println!(
        "Todo(id={}, title={}, completed={})",
        todo.id, todo.title, todo.completed
    )
}

#[allow(unused)]
fn print_todos(mut todos: Vec<&TodoJson>) {
    todos.sort_by_key(|todo| todo.id);

    println!("--------------------------");

    for todo in todos {
        print_todo(todo)
    }
    println!("--------------------------");
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(state::AppState::new()));

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
