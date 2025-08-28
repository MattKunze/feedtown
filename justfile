default:
  just --list

# run the API
api:
  cargo run --bin api

# run migrations to update database
migrate:
  cargo run --bin migration -- up