use anyhow::Result;
use sea_orm::{Database, DatabaseConnection, DbErr, RuntimeErr};
use std::env;

/// Database connection configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

impl DatabaseConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        
        Ok(Self {
            host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid DB_PORT"))?,
            username: env::var("DB_USERNAME")
                .map_err(|_| anyhow::anyhow!("DB_USERNAME must be set"))?,
            password: env::var("DB_PASSWORD")
                .map_err(|_| anyhow::anyhow!("DB_PASSWORD must be set"))?,
            database_name: env::var("DB_NAME")
                .map_err(|_| anyhow::anyhow!("DB_NAME must be set"))?,
        })
    }
    
    /// Build the database URL from configuration
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

/// Create a new database connection pool
pub async fn create_connection() -> Result<DatabaseConnection, DbErr> {
    let config = DatabaseConfig::from_env()
        .map_err(|e| DbErr::Conn(RuntimeErr::Internal(format!("Configuration error: {}", e))))?;
    
    let database_url = config.database_url();
    Database::connect(&database_url).await
}

/// Create a connection with custom URL
pub async fn create_connection_with_url(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}