use anyhow::Result;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};

pub mod connection;

pub use connection::*;

/// Initialize the database connection using environment variables
pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    dotenvy::dotenv().ok();

    let config = connection::DatabaseConfig::from_env().unwrap();
    Database::connect(&config.database_url()).await
}

/// Test the database connection
pub async fn test_connection() -> Result<()> {
    let db = establish_connection().await?;

    // Simple ping to test connection
    let statement =
        Statement::from_string(sea_orm::DatabaseBackend::Postgres, "SELECT 1".to_string());

    db.execute(statement).await?;

    println!("Database connection successful!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env::with_vars;

    #[test]
    fn test_database_config_from_env() {
        with_vars(
            [
                ("DB_HOST", Some("test_host")),
                ("DB_PORT", Some("5433")),
                ("DB_USERNAME", Some("test_user")),
                ("DB_PASSWORD", Some("test_pass")),
                ("DB_NAME", Some("test_db")),
            ],
            || {
                let config = connection::DatabaseConfig::from_env().unwrap();

                assert_eq!(config.host, "test_host");
                assert_eq!(config.port, 5433);
                assert_eq!(config.username, "test_user");
                assert_eq!(config.password, "test_pass");
                assert_eq!(config.database_name, "test_db");
            },
        );
    }

    #[test]
    fn test_database_url_generation() {
        let config = connection::DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "user".to_string(),
            password: "pass".to_string(),
            database_name: "testdb".to_string(),
        };

        let url = config.database_url();
        assert_eq!(url, "postgres://user:pass@localhost:5432/testdb");
    }

    #[test]
    fn test_database_config_defaults() {
        with_vars(
            [
                ("DB_HOST", None),
                ("DB_PORT", None),
                ("DB_USERNAME", Some("test_user_defaults")),
                ("DB_PASSWORD", Some("test_pass_defaults")),
                ("DB_NAME", Some("test_db_defaults")),
            ],
            || {
                let config = connection::DatabaseConfig::from_env().unwrap();

                assert_eq!(config.host, "localhost");
                assert_eq!(config.port, 5432);
                assert_eq!(config.username, "test_user_defaults");
                assert_eq!(config.password, "test_pass_defaults");
                assert_eq!(config.database_name, "test_db_defaults");
            },
        );
    }
}
