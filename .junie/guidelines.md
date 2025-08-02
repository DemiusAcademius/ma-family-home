# My Family Home Core Development Guidelines

This document provides guidelines and instructions for developing and working with the Luninair Core project.

## Build and Configuration Instructions

### Prerequisites

- Rust (latest stable version)
- PostgreSQL 17.5 or compatible version
- Docker and Docker Compose (optional, for containerized development)

### Local Development Setup

1. **Clone the repository**

2. **Build the project**

   ```powershell
   cargo build
   ```

3. **Run the project**

   ```powershell
   cargo run
   ```

### Docker Deployment

To build and run the entire application using Docker Compose:

```powershell
docker-compose up -d
```

This will:
2. Build and start the application container

The API will be accessible at http://localhost:8080

## Testing Information

### Running Tests

To run all tests:

```powershell
cargo test
```

To run a specific test:

```powershell
cargo test test_name
```
