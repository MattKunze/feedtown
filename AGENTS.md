# Feedtown Development Guide for AI Agents

## Project Overview
Feedtown is a modular Rust application built using Cargo workspaces for scalability with individual crates for different components, following modern Rust best practices with Axum for HTTP APIs.

## Build Commands
- `cargo build` - Build all workspace crates
- `cargo run --bin api` - Run API server on port 3000
- `cargo run --bin migration -- up` - Run database migrations
- `just api` - Quick API start (via justfile)
- `just migrate` - Quick migration run

## Testing
- `cargo test` - Run all tests
- `cargo test <testname>` - Run specific test by name pattern
- `cargo test --package api` - Run tests for specific crate
- Write unit tests in same file, integration tests in `tests/` directory

## Linting & Formatting
- `just lint-fix` - Auto-fix clippy issues and format code
- `cargo clippy` - Run lints
- `cargo fmt` - Format code
- **CRITICAL**: Always run after any Rust code changes

## Project Structure
- **Root**: Workspace configuration with shared dependencies (only for truly cross-crate dependencies)
- **api/**: HTTP API server built with Axum framework, health check at `/health`
- **migration/**: Database migration tooling with Sea-ORM
- Use workspace dependencies ONLY for truly shared dependencies (tokio, tracing, serde, dotenvy)
- Keep crate-specific dependencies isolated in each crate's Cargo.toml

## Code Style & Architecture
- Standard Rust formatting (cargo fmt), follow Rust naming conventions
- Import grouping: std libs, external crates, internal modules
- PascalCase for types/structs, snake_case for functions/variables
- Return `Result<T, E>` for fallible operations, prefer explicit error handling
- Use `tracing` for structured logging, not println!
- Tower middleware for cross-cutting concerns (tracing, CORS, authentication)
- JSON serialization with serde, proper HTTP status codes
- Follow async/await patterns with tokio

## Dependencies
- **Shared Dependencies** (workspace): tokio, tracing, tracing-subscriber, serde, serde_json, dotenvy
- **API-specific**: axum, tower, tower-http
- **Migration-specific**: sea-orm, sea-orm-migration, async-std
- Always specify features explicitly, avoid heavyweight dependencies

## Adding New Crates
1. Create new directory: `mkdir crate-name` (kebab-case naming)
2. Add to workspace members in root `Cargo.toml`
3. Create crate's `Cargo.toml` inheriting workspace properties
4. Only add dependencies to workspace if truly shared across multiple crates