use std::sync::{Arc, Mutex};

mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(state::AppState::new()));

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
