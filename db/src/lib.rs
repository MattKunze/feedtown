use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;
use thiserror::Error;
use tracing::{error, info};

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database connection error: {0}")]
    Connection(#[from] DbErr),
    #[error("Environment variable error: {0}")]
    EnvVar(String),
}

pub type DbResult<T> = Result<T, DbError>;

#[derive(Clone)]
pub struct DbManager {
    connection: DatabaseConnection,
}

impl DbManager {
    pub async fn new() -> DbResult<Self> {
        dotenvy::dotenv().ok();

        let database_url = Self::build_database_url()?;

        info!("Connecting to database...");
        let connection = Database::connect(&database_url).await?;

        info!("Database connection established");
        Ok(Self { connection })
    }

    pub async fn health_check(&self) -> DbResult<bool> {
        match self.connection.ping().await {
            Ok(_) => {
                info!("Database health check passed");
                Ok(true)
            }
            Err(e) => {
                error!("Database health check failed: {}", e);
                Err(DbError::Connection(e))
            }
        }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    fn build_database_url() -> DbResult<String> {
        if let Ok(url) = env::var("DATABASE_URL") {
            return Ok(url);
        }

        let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());

        let username = env::var("DB_USERNAME")
            .map_err(|_| DbError::EnvVar("DB_USERNAME must be set".to_string()))?;
        let password = env::var("DB_PASSWORD")
            .map_err(|_| DbError::EnvVar("DB_PASSWORD must be set".to_string()))?;
        let database =
            env::var("DB_NAME").map_err(|_| DbError::EnvVar("DB_NAME must be set".to_string()))?;

        Ok(format!(
            "postgresql://{username}:{password}@{host}:{port}/{database}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_url_from_env_vars() {
        env::set_var("DB_HOST", "testhost");
        env::set_var("DB_PORT", "5433");
        env::set_var("DB_USERNAME", "testuser");
        env::set_var("DB_PASSWORD", "testpass");
        env::set_var("DB_NAME", "testdb");
        env::remove_var("DATABASE_URL");

        let url = DbManager::build_database_url().unwrap();
        assert_eq!(url, "postgresql://testuser:testpass@testhost:5433/testdb");
    }

    #[tokio::test]
    async fn test_database_url_direct() {
        env::set_var("DATABASE_URL", "postgresql://direct:pass@host:5432/db");

        let url = DbManager::build_database_url().unwrap();
        assert_eq!(url, "postgresql://direct:pass@host:5432/db");
    }
}
