# API Project

A simple REST API built with Axum framework in Rust.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Running the Server

```bash
cargo run -p api
```

The server will start on `http://localhost:3000`

### Endpoints

- `GET /hello` - Returns a simple hello world message

Example response:
```json
{
  "message": "Hello, World!"
}
```

### Testing

You can test the endpoint using curl:

```bash
curl http://localhost:3000/hello
```

## Dependencies

- **axum**: Web framework
- **tokio**: Async runtime
- **tower**: Service abstraction layer
- **serde**: Serialization framework
- **serde_json**: JSON support