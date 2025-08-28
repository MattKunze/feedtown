# Feedtown Project Instructions for OpenCode/Claude

> **Note**: Consolidated AI assistant instructions are now maintained in `AI_INSTRUCTIONS.md`. This file serves as a reference pointing to the main instruction file.

For comprehensive project guidelines, development patterns, and tool-specific instructions, see: **[AI_INSTRUCTIONS.md](./AI_INSTRUCTIONS.md)**

## Quick Reference

### Common Commands
- `cargo run --bin api` - Run the API server
- `cargo build` - Build all workspace crates
- `cargo test` - Run all tests
- `cargo fmt` - Format code
- `cargo clippy` - Run lints

### Key Architecture Notes
- Cargo workspace with modular crates
- Axum framework for HTTP APIs
- Structured logging with tracing
- Workspace dependencies for version consistency