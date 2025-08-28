use sea_orm_migration::prelude::*;
use std::env;

#[async_std::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    // Construct DATABASE_URL from individual DB_* environment variables
    if env::var("DATABASE_URL").is_err() {
        let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
        let username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        let database = env::var("DB_NAME").expect("DB_NAME must be set");
        
        let database_url = format!("postgresql://{}:{}@{}:{}/{}", username, password, host, port, database);
        env::set_var("DATABASE_URL", database_url);
    }
    
    cli::run_cli(migration::Migrator).await;
}