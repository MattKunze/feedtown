# Database Migrations

This crate handles database migrations using sea-orm-migrate.

## Setup

1. Make sure your `.env` file is configured with the database connection details:
   ```
   DB_HOST=localhost
   DB_PORT=5432
   DB_USERNAME=feedtown
   DB_PASSWORD=your_password
   DB_NAME=feedtown
   ```

2. The migration system will automatically read the `.env` file and construct the `DATABASE_URL`.

## Usage

### Apply all pending migrations
```bash
cargo run --bin migration -- up
```

### Check migration status
```bash
cargo run --bin migration -- status
```

### Create a new migration
```bash
cargo run --bin migration -- generate new_migration_name
```

### Reset all migrations (drops all tables and reapplies)
```bash
cargo run --bin migration -- fresh
```

### Rollback migrations
```bash
cargo run --bin migration -- down
```

## Included Migrations

### m20240827_000001_create_user_table
Creates a `user` table with:
- `id` (auto-incrementing primary key)
- `username` (unique string)
- `email` (unique string)
- `created_at` (timestamp with timezone)
- `updated_at` (timestamp with timezone)

## Environment Variables

The migration tool expects these environment variables (from `.env`):
- `DB_HOST` - Database host (default: localhost)
- `DB_PORT` - Database port (default: 5432)  
- `DB_USERNAME` - Database username
- `DB_PASSWORD` - Database password
- `DB_NAME` - Database name

These are automatically combined into a `DATABASE_URL` in the format:
`postgresql://username:password@host:port/database_name`