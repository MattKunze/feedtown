use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;

async fn hello() -> Json<Value> {
    Json(json!({ "message": "Hello, World!" }))
}

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/hello", get(hello));

    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}