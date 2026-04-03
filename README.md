# Rust Test Application

Simple Rust web application using Actix-web for deployment testing.

## Features

- RESTful API with Actix-web
- JSON responses
- CORS support
- Logging
- Health check endpoint
- In-memory data storage

## Endpoints

- `GET /` - Root endpoint
- `GET /health` - Health check
- `GET /items` - List all items
- `POST /items` - Create item
- `GET /items/{id}` - Get item by ID
- `GET /stats` - Application statistics

## Local Development

```bash
# Build the application
cargo build

# Run the application
cargo run

# Run in release mode
cargo run --release
```

## Docker

```bash
# Build image
docker build -t rust-app .

# Run container
docker run -p 8080:8080 rust-app
```

## Testing

```bash
# Health check
curl http://localhost:8080/health

# Create item
curl -X POST http://localhost:8080/items \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Item","price":19.99,"quantity":5}'

# Get all items
curl http://localhost:8080/items
```

## Deployment

This application is designed to be deployed on:
- AWS ECS/Fargate
- AWS EC2
- Docker containers
- Kubernetes

Port: 8080

## Performance

Rust provides excellent performance characteristics:
- Low memory footprint
- Fast startup time
- High throughput
- Efficient resource usage
