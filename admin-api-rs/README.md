# Admin API (Rust)

This is the Rust implementation of the Admin API service for SaaSExpress.

## Features

- RESTful API using Axum
- ORM using Diesel with SQLite
- OpenAPI documentation with Swagger UI
- CORS support
- Structured logging

## Prerequisites

- Rust (1.76+)
- SQLite development libraries

## Setup

1. Install Rust and Cargo (if not already installed):
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Diesel CLI:
   ```
   cargo install diesel_cli --no-default-features --features sqlite
   ```

3. Setup the database:
   ```
   diesel setup
   diesel migration run
   ```

## Running

### Development Mode

```
make run-dev
```
or
```
cargo watch -x run
```

### Production Mode

```
make run
```
or
```
cargo run --release
```

## Docker

Build the Docker image:
```
make docker
```

Run the Docker container:
```
make docker-run
```

## API Documentation

Once the service is running, you can access the Swagger UI documentation at:
```
http://localhost:8081/api/docs/
```

## Project Structure

- **src/api/** - API handlers and routing
- **src/db/** - Database connection and repositories
- **src/models/** - Database and API models
- **src/schema.rs** - Generated database schema
- **migrations/** - Database migrations

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -am 'Add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Create a new Pull Request

## License

This project is licensed under the Apache License - see the LICENSE file for details.