# Feedtown

A modular Rust application built with workspaces for scalable development.

## Architecture

This project uses Cargo workspaces to organize code into separate crates:

- `api` - HTTP API server built with Axum framework

## Getting Started

### Prerequisites

- Rust 1.70+ 
- Cargo

### Running the API

```bash
# Run the API server
cargo run --bin api

# The server will start on http://localhost:3000
# Health check available at http://localhost:3000/health
```

### Development

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Check code formatting
cargo fmt

# Run clippy lints
cargo clippy
```

## Project Structure

```
feedtown/
├── Cargo.toml          # Workspace configuration
├── api/                # API server crate
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── README.md
```

## Adding New Crates

To add a new crate to the workspace:

1. Create the crate directory: `mkdir new-crate`
2. Add it to the workspace members in the root `Cargo.toml`
3. Create the crate's `Cargo.toml` and source files

## License

MIT OR Apache-2.0