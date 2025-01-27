# Rust Database API

A RESTful API built with Rust, Actix-web, and PostgreSQL.

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- SQLx CLI (optional, for migrations)

## Setup

1. Create a PostgreSQL database:
```sql
CREATE DATABASE rust_api_db;
```

2. Copy the `.env.example` to `.env` and update the database connection string:
```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/rust_api_db
RUST_LOG=debug
```

3. Install dependencies and run migrations:
```bash
cargo install sqlx-cli
sqlx migrate run
```

4. Run the server:
```bash
cargo run
```

## API Endpoints

- `GET /health` - Health check endpoint
- `POST /items` - Create a new item
- `GET /items` - List all items
- `GET /items/{id}` - Get a specific item by ID

### Example Requests

Create an item:
```bash
curl -X POST http://localhost:8080/items \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Item", "description": "This is a test item"}'
```

List all items:
```bash
curl http://localhost:8080/items
```

Get a specific item:
```bash
curl http://localhost:8080/items/1
```

## Development

To run in development mode with auto-reload:
```bash
cargo watch -x run
```
