# Database Crate (db)

This crate provides database connectivity and utilities for the Feedtown application using Sea-ORM with PostgreSQL.

## Features

- PostgreSQL database connection management
- Environment-based configuration
- Connection pooling via Sea-ORM
- Health check utilities

## Setup

1. Install PostgreSQL on your system
2. Create a database for the application
3. Copy `.env.example` to `.env` and update the database configuration:

```bash
cp .env.example .env
```

4. Edit `.env` with your database credentials:

```env
DB_HOST=localhost
DB_PORT=5432
DB_USERNAME=your_username
DB_PASSWORD=your_password
DB_NAME=feedtown_db
```

## Usage

### Basic Connection

```rust
use db::establish_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = establish_connection().await?;
    // Use the database connection
    Ok(())
}
```

### Using Configuration

```rust
use db::{DatabaseConfig, create_connection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = DatabaseConfig::from_env()?;
    let db = create_connection().await?;
    // Use the database connection
    Ok(())
}
```

### Health Check

```rust
use db::test_connection;

async fn check_db_health() {
    match test_connection().await {
        Ok(_) => println!("Database is healthy"),
        Err(e) => eprintln!("Database error: {}", e),
    }
}
```

## Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DB_HOST` | Database host | localhost | No |
| `DB_PORT` | Database port | 5432 | No |
| `DB_USERNAME` | Database username | - | Yes |
| `DB_PASSWORD` | Database password | - | Yes |
| `DB_NAME` | Database name | - | Yes |

## Dependencies

- `sea-orm`: ORM and database toolkit
- `tokio`: Async runtime
- `anyhow`: Error handling
- `dotenvy`: Environment variable loading
- `serde`: Serialization support

## Development

To test the database connection:

```bash
cargo test -p db
```

To run with database logging:

```bash
RUST_LOG=sea_orm=debug cargo run
```
