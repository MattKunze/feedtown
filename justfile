default:
  just --list

[parallel]
app: api web

# run the API
api:
  cargo run --bin api

# run migrations to update database
migrate:
  cargo run --bin migration -- up

# run the web development server
web:
  cd web && deno task dev

# build the web application
web-build:
  cd web && deno task build

# preview the built web application
web-preview:
  cd web && deno task preview

# Rust-specific tasks
lint-rust:
  cargo clippy --all-targets --all-features

fmt-rust:
  cargo fmt

fmt-check-rust:
  cargo fmt --check

test-rust:
  cargo test

# Deno/Web-specific tasks
lint-deno:
  cd web && deno task lint

fmt-deno:
  cd web && deno task fmt

fmt-check-deno:
  cd web && deno task fmt-check

typecheck-deno:
  cd web && deno task typecheck

test-deno:
  cd web && deno task test

# Combined tasks that run across the full repo
[parallel]
lint: lint-rust lint-deno

[parallel]
fmt: fmt-rust fmt-deno

[parallel]
fmt-check: fmt-check-rust fmt-check-deno

typecheck: typecheck-deno
  cargo check --all-targets --all-features

[parallel]
test: test-rust test-deno

# Legacy combined task for backwards compatibility
lint-fix: lint-rust fmt-rust
  cargo clippy --fix --allow-dirty