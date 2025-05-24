use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use db::{establish_connection, test_connection};

async fn hello() -> Json<Value> {
    Json(json!({ "message": "Hello, World!" }))
}

async fn health_check() -> Json<Value> {
    match test_connection().await {
        Ok(_) => Json(json!({ 
            "status": "healthy", 
            "database": "connected" 
        })),
        Err(e) => Json(json!({ 
            "status": "unhealthy", 
            "database": "disconnected",
            "error": e.to_string()
        })),
    }
}

#[tokio::main]
async fn main() {
    // Test database connection on startup
    match establish_connection().await {
        Ok(_) => println!("✅ Database connection established"),
        Err(e) => {
            eprintln!("❌ Database connection failed: {}", e);
            eprintln!("Make sure to configure your .env file with correct database settings");
        }
    }

    // Build our application with routes
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/health", get(health_check));

    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}