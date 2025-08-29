use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use db::{
    models::{CreateUserRequest, UpdateUserRequest},
    user,
};
use serde_json::{json, Value};

use crate::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<user::Model>, StatusCode> {
    let user_service = state.db.user_service();

    match user_service.create_user(payload).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            tracing::error!("Failed to create user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<user::Model>>, StatusCode> {
    let user_service = state.db.user_service();

    match user_service.get_users().await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            tracing::error!("Failed to fetch users: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<user::Model>, StatusCode> {
    let user_service = state.db.user_service();

    match user_service.get_user_by_id(id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<user::Model>, StatusCode> {
    let user_service = state.db.user_service();

    match user_service.update_user(id, payload).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    let user_service = state.db.user_service();

    match user_service.delete_user(id).await {
        Ok(true) => Ok(Json(json!({"message": "User deleted successfully"}))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to delete user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
