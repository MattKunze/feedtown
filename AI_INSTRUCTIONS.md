# AI Assistant Instructions for Feedtown

## Project Overview

Feedtown is a modular Rust application built using Cargo workspaces. The project is designed for scalability with individual crates for different components, following modern Rust best practices with Axum for HTTP APIs.

## Project Structure

- **Root**: Workspace configuration with shared dependencies (only for truly cross-crate dependencies)
- **api/**: HTTP API server built with Axum framework
- **migration/**: Database migration tooling with Sea-ORM
- Future crates will be added as separate workspace members

## Development Guidelines

### Code Style & Conventions
- Follow standard Rust conventions and idioms
- Use `cargo fmt` for formatting - standard Rust formatting
- Run `cargo clippy` for linting
- Prefer explicit error handling over panics with Result types
- Use structured logging with the `tracing` crate
- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Write descriptive comments for public APIs

### Architecture Principles
- Workspace-based modular architecture with each crate having a single, well-defined responsibility
- Prefer dependency injection and trait-based abstractions
- Use workspace dependencies ONLY for truly shared dependencies across multiple crates
- Keep crate-specific dependencies isolated to maintain clear separation of concerns
- Keep external dependencies minimal and well-justified
- Tower middleware for cross-cutting concerns (tracing, CORS, authentication)

### API Development (api crate)
- Built with Axum framework using extractors and response types
- Uses tower middleware for cross-cutting concerns (tracing, CORS)
- JSON serialization with serde
- Structured error responses with custom error types
- Health check endpoint at `/health`
- Include proper HTTP status codes

### Testing
- Write unit tests in the same file as the code being tested
- Integration tests in `tests/` directory
- Use `cargo test` to run all tests
- Mock external dependencies in tests

### Dependencies & Common Patterns
- **Key Dependencies**: axum (web framework), tokio (async runtime), tower/tower-http (middleware), tracing (structured logging), serde (serialization)
- **Dependency Organization**: Use workspace dependencies ONLY for truly shared dependencies across multiple crates (tokio, tracing, serde, dotenvy). Crate-specific dependencies should be declared directly in each crate's Cargo.toml
- **Shared Dependencies** (keep in workspace): tokio, tracing, tracing-subscriber, serde, serde_json, dotenvy
- **API-specific Dependencies** (api/Cargo.toml): axum, tower, tower-http
- **Migration-specific Dependencies** (migration/Cargo.toml): sea-orm, sea-orm-migration, async-std
- Always specify features explicitly
- Avoid heavyweight dependencies unless necessary
- State management with Axum extractors
- Follow async/await patterns with tokio
- JSON request/response handling

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
5. **Dependencies**: Only add dependencies to workspace if they are truly shared across multiple crates. Otherwise, declare dependencies directly in the crate's Cargo.toml with explicit versions and features

## Tool-Specific Guidance

### For Code Generation (GitHub Copilot)
When suggesting code:
- Use workspace dependencies (.workspace = true) ONLY for shared dependencies (tokio, tracing, serde, dotenvy)
- Declare crate-specific dependencies directly with explicit versions
- Include proper error handling with Result types
- Add appropriate logging statements with tracing
- Follow async/await patterns with tokio
- Use Axum's extractors and response types
- Include proper HTTP status codes
- File organization: api/src/main.rs for main API server

### For Development Tasks (Claude/OpenCode)
- Always run `cargo fmt` and `cargo clippy` after making changes
- Test changes with `cargo test`
- Follow the testing approach outlined above
- Use the common commands reference for development workflow
- When adding features, consider the modular workspace architecture

## Current State
- Basic workspace setup complete
- API crate with health check endpoint
- Ready for feature development and additional crates