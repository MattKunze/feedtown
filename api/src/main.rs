use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use db::DbManager;
use serde::Serialize;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_handler,
        handlers::create_user,
        handlers::list_users,
        handlers::get_user,
        handlers::update_user,
        handlers::delete_user,
    ),
    components(
        schemas(HealthResponse, DatabaseHealth, db::models::CreateUserRequest, db::models::UpdateUserRequest, db::user::Model)
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "users", description = "User management endpoints")
    ),
    info(
        title = "Feedtown API",
        description = "A REST API for the Feedtown application",
        version = "0.1.0"
    )
)]
struct ApiDoc;

#[derive(Clone)]
struct AppState {
    db: DbManager,
}

#[derive(Serialize, utoipa::ToSchema)]
struct HealthResponse {
    status: String,
    version: String,
    database: DatabaseHealth,
}

#[derive(Serialize, utoipa::ToSchema)]
struct DatabaseHealth {
    status: String,
    message: Option<String>,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Health check response", body = HealthResponse),
        (status = 503, description = "Service degraded")
    )
)]
async fn health_handler(State(state): State<AppState>) -> Result<Json<HealthResponse>, StatusCode> {
    let (db_status, db_message) = match state.db.health_check().await {
        Ok(_) => ("healthy".to_string(), None),
        Err(e) => ("unhealthy".to_string(), Some(e.to_string())),
    };

    let overall_status = if db_status == "healthy" {
        "healthy"
    } else {
        "degraded"
    };

    let response = HealthResponse {
        status: overall_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: DatabaseHealth {
            status: db_status,
            message: db_message,
        },
    };
    Ok(Json(response))
}

fn create_router(db: DbManager) -> Router {
    let state = AppState { db };

    Router::new()
        .route("/health", get(health_handler))
        .nest("/api", handlers::user_routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing database connection...");
    let db = DbManager::new().await?;
    tracing::info!("Database connection initialized");

    let app = create_router(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
