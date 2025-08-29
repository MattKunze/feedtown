use crate::models::{self, CreateUserRequest, UpdateUserRequest};
use crate::{DbError, DbResult};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use tracing::{error, info};

pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> DbResult<models::user::Model> {
        info!("Creating user with username: {}", request.username);

        let now = Utc::now();
        let user = models::user::ActiveModel {
            username: Set(request.username),
            email: Set(request.email),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        match user.insert(&self.db).await {
            Ok(user) => {
                info!("User created successfully with id: {}", user.id);
                Ok(user)
            }
            Err(e) => {
                error!("Failed to create user: {}", e);
                Err(DbError::Connection(e))
            }
        }
    }

    pub async fn get_user_by_id(&self, id: i32) -> DbResult<Option<models::user::Model>> {
        info!("Fetching user with id: {}", id);

        match models::user::Entity::find_by_id(id).one(&self.db).await {
            Ok(user) => {
                if user.is_some() {
                    info!("User found with id: {}", id);
                } else {
                    info!("No user found with id: {}", id);
                }
                Ok(user)
            }
            Err(e) => {
                error!("Failed to fetch user with id {}: {}", id, e);
                Err(DbError::Connection(e))
            }
        }
    }

    pub async fn get_users(&self) -> DbResult<Vec<models::user::Model>> {
        info!("Fetching all users");

        match models::user::Entity::find().all(&self.db).await {
            Ok(users) => {
                info!("Found {} users", users.len());
                Ok(users)
            }
            Err(e) => {
                error!("Failed to fetch users: {}", e);
                Err(DbError::Connection(e))
            }
        }
    }

    pub async fn update_user(
        &self,
        id: i32,
        request: UpdateUserRequest,
    ) -> DbResult<Option<models::user::Model>> {
        info!("Updating user with id: {}", id);

        let user = match models::user::Entity::find_by_id(id).one(&self.db).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                info!("No user found with id: {}", id);
                return Ok(None);
            }
            Err(e) => {
                error!("Failed to fetch user for update: {}", e);
                return Err(DbError::Connection(e));
            }
        };

        let mut active_user: models::user::ActiveModel = user.into();

        if let Some(username) = request.username {
            active_user.username = Set(username);
        }
        if let Some(email) = request.email {
            active_user.email = Set(email);
        }
        active_user.updated_at = Set(Utc::now());

        match active_user.update(&self.db).await {
            Ok(user) => {
                info!("User updated successfully with id: {}", user.id);
                Ok(Some(user))
            }
            Err(e) => {
                error!("Failed to update user: {}", e);
                Err(DbError::Connection(e))
            }
        }
    }

    pub async fn delete_user(&self, id: i32) -> DbResult<bool> {
        info!("Deleting user with id: {}", id);

        match models::user::Entity::delete_by_id(id).exec(&self.db).await {
            Ok(result) => {
                let deleted = result.rows_affected > 0;
                if deleted {
                    info!("User deleted successfully with id: {}", id);
                } else {
                    info!("No user found to delete with id: {}", id);
                }
                Ok(deleted)
            }
            Err(e) => {
                error!("Failed to delete user with id {}: {}", id, e);
                Err(DbError::Connection(e))
            }
        }
    }
}
