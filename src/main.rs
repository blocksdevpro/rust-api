use std::sync::{Arc, Mutex};

mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(state::AppState::new()));

    let app = routes::create_router(state);
    let addr = "0.0.0.0:3000";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server running at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
