# Feedtown Project Instructions for OpenCode/Claude

## Project Overview

Feedtown is a modular Rust application built using Cargo workspaces. The project is designed for scalability with individual crates for different components.

## Project Structure

- **Root**: Workspace configuration with shared dependencies
- **api/**: HTTP API server built with Axum framework
- Future crates will be added as separate workspace members

## Development Guidelines

### Code Style
- Follow standard Rust conventions and idioms
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Prefer explicit error handling over panics
- Use structured logging with the `tracing` crate

### Architecture Principles
- Each crate should have a single, well-defined responsibility
- Prefer dependency injection and trait-based abstractions
- Use workspace dependencies to maintain version consistency
- Keep external dependencies minimal and well-justified

### API Development (api crate)
- Built with Axum framework
- Uses tower middleware for cross-cutting concerns (tracing, CORS)
- JSON serialization with serde
- Structured error responses
- Health check endpoint at `/health`

### Testing
- Write unit tests in the same file as the code being tested
- Integration tests in `tests/` directory
- Use `cargo test` to run all tests
- Mock external dependencies in tests

### Common Commands
- `cargo run --bin api` - Run the API server
- `cargo build` - Build all workspace crates
- `cargo test` - Run all tests
- `cargo fmt` - Format code
- `cargo clippy` - Run lints

### Adding New Crates
1. Create new directory: `mkdir crate-name`
2. Add to workspace members in root `Cargo.toml`
3. Create crate's `Cargo.toml` inheriting workspace properties
4. Follow naming convention: kebab-case for crate names

### Dependencies
- Prefer workspace dependencies defined in root `Cargo.toml`
- Common dependencies: tokio, axum, serde, tracing
- Always specify features explicitly
- Avoid heavyweight dependencies unless necessary

## Current State
- Basic workspace setup complete
- API crate with health check endpoint
- Ready for feature development and additional crates