# Feedtown Development Guide for AI Agents

## Project Overview
Feedtown is a modular full-stack application with:
- **Backend**: Rust with Cargo workspaces, Axum for HTTP APIs
- **Frontend**: TypeScript/React with Deno runtime, Vite bundler, Tailwind CSS + DaisyUI

## Build Commands
- `cargo build` - Build all Rust workspace crates
- `cargo run --bin api` - Run API server on port 3000
- `cargo run --bin migration -- up` - Run database migrations
- `just api` - Quick API start (via justfile)
- `just web` - Run web development server on port 5173
- `just app` - Run both API and web servers in parallel
- `just migrate` - Quick migration run

## Testing
- **Rust**: `cargo test` or `just test-rust`
- **Deno/Web**: `deno task test` or `just test-deno`
- **Combined**: `just test` (runs both in parallel)
- Write unit tests in same file (Rust), integration tests in `tests/` directory
- Deno tests use standard library assertions from `https://deno.land/std/assert/`

## Linting & Formatting - **CRITICAL REQUIREMENT**
**MANDATORY**: After ANY code changes, you MUST run formatting and linting commands. All warnings MUST be addressed before considering the task complete.

### Language-Specific Commands:
- **Rust**: `just lint-rust`, `just fmt-rust`, `just fmt-check-rust`
- **Deno/Web**: `just lint-deno`, `just fmt-deno`, `just fmt-check-deno`, `just typecheck-deno`

### Combined Commands (run both languages):
- `just lint` - Lint all code (Rust + Deno)
- `just fmt` - Format all code (Rust + Deno)  
- `just fmt-check` - Check formatting without applying changes
- `just typecheck` - Type check all code

### Legacy Commands:
- `just lint-fix` - Auto-fix Rust clippy issues and format Rust code

### **CRITICAL WORKFLOW**:
1. Make code changes
2. **ALWAYS** run `just fmt` to format code
3. **ALWAYS** run `just lint` to check for issues
4. **MUST** address ALL warnings and errors before proceeding
5. Run `just typecheck` for type safety verification
6. Only then consider the task complete

## Project Structure
- **Root**: Workspace configuration with shared dependencies (only for truly cross-crate dependencies)
- **api/**: HTTP API server built with Axum framework, health check at `/health`
- **migration/**: Database migration tooling with Sea-ORM
- **web/**: React/TypeScript frontend with Deno runtime, Vite bundler
- Use workspace dependencies ONLY for truly shared dependencies (tokio, tracing, serde, dotenvy)
- Keep crate-specific dependencies isolated in each crate's Cargo.toml

## Code Style & Architecture

### Rust:
- Standard Rust formatting (cargo fmt), follow Rust naming conventions
- Import grouping: std libs, external crates, internal modules
- PascalCase for types/structs, snake_case for functions/variables
- Return `Result<T, E>` for fallible operations, prefer explicit error handling
- Use `tracing` for structured logging, not println!
- Tower middleware for cross-cutting concerns (tracing, CORS, authentication)
- JSON serialization with serde, proper HTTP status codes
- Follow async/await patterns with tokio

### TypeScript/React (Deno):
- Use explicit file extensions for local imports (`.tsx`, `.ts`)
- Follow React/JSX best practices, prefer functional components
- Use Tailwind CSS classes, DaisyUI components for styling
- Type all props and state with TypeScript
- Follow Deno conventions: use `globalThis` instead of Node.js globals
- Import external dependencies via npm: specifiers in deno.jsonc

## Dependencies

### Rust:
- **Shared Dependencies** (workspace): tokio, tracing, tracing-subscriber, serde, serde_json, dotenvy
- **API-specific**: axum, tower, tower-http
- **Migration-specific**: sea-orm, sea-orm-migration, async-std
- Always specify features explicitly, avoid heavyweight dependencies

### Deno/Web:
- React, React Router, TypeScript types managed via npm: imports in deno.jsonc
- Vite for bundling, Tailwind CSS + DaisyUI for styling
- Use Deno standard library for testing and utilities
- PostCSS + Autoprefixer for CSS processing

## Adding New Crates
1. Create new directory: `mkdir crate-name` (kebab-case naming)
2. Add to workspace members in root `Cargo.toml`
3. Create crate's `Cargo.toml` inheriting workspace properties
4. Only add dependencies to workspace if truly shared across multiple crates

## **CRITICAL REMINDERS**
- **NEVER** skip linting and formatting after code changes
- **ALL** warnings must be addressed, not ignored
- Run `just fmt && just lint` after every modification
- Use `just typecheck` to catch type errors early
- Test both individual components (`just test-rust`, `just test-deno`) and full system (`just test`)
- Follow language-specific conventions (Rust naming, Deno explicit imports, React patterns)