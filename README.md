# Auth-Service

A high-performance, secure, and scalable Authentication Service built with **Rust**. This project leverages a **Clean Architecture** approach to separate concerns, ensuring maintainability and testability. It supports both **REST** and **gRPC** interfaces, backed by a robust database layer.

## Features

-   **Dual Interface**: Exposes both a RESTful API and gRPC endpoints for flexible integration.
-   **Clean Architecture**: Separation of concerns into handlers, services, repositories, and models.
-   **Security**: JWT-based authentication and role-based access control.
-   **Scalability**: Dockerized and Kubernetes-ready.
-   **Observability**: Integrated logging and middleware.

## Technology Stack

-   **Language**: [Rust](https://www.rust-lang.org/) (2024 Edition)
-   **Containerization**: Docker & Docker Compose
-   **Orchestration**: Kubernetes
-   **Configuration**: Environment variables (`dotenv`)

## Project Structure

The project follows a modular structure to keep the codebase organized:

```
src/
├── config/         # Application configuration and environment variable parsing
├── db/             # Database connection pooling and initialization
├── dto/            # Data Transfer Objects (request/response schemas)
├── errors/         # Centralized error handling and custom error types
├── grpc/           # gRPC service implementations and generated code
├── handlers/       # HTTP request handlers (controllers)
├── middleware/     # Request processing (Auth, CORS, Logging)
├── models/         # Core domain models and database entities
├── repository/     # Data access layer (CRUD operations)
├── routes/         # HTTP route definitions and registration
├── services/       # Core business logic
└── utils/          # Helper functions (JWT generation, hashing, etc.)
proto/              # Protocol Buffers definitions for gRPC
migrations/         # Database migration scripts
scripts/            # Utility scripts for build/deploy
```

## Getting Started

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) (latest stable)
-   [Docker](https://www.docker.com/) & Docker Compose
-   [kubectl](https://kubernetes.io/docs/tasks/tools/) (optional, for K8s deployment)

### 1. Environment Setup

Create a `.env` file in the root directory (or copy the example):

```bash
cp .env.example .env
```

Ensure the following variables are set:

```env
JWT_SECRET=your_secure_secret
ALLOWED_HOST=*
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/auth-service
ENVIRONMENT=development
```

### 2. Running Locally (Docker Compose)

The easiest way to run the service is via Docker Compose, which handles the application and any potential dependencies (like a DB).

```bash
docker-compose up --build
```

The service will be available at `http://localhost:8080`.

### 3. Running Locally (Cargo)

If you prefer running directly on your machine:

```bash
# Build the project
cargo build --release

# Run the project
cargo run
```

### 4. Kubernetes Deployment

To deploy to a Kubernetes cluster:

1.  **Create Secrets**:
    ```bash
    kubectl create secret generic auth-service-secrets --from-env-file=.env
    ```

2.  **Apply Manifests**:
    ```bash
    kubectl apply -f k8.yaml
    ```

## Testing

Run the test suite to ensure everything is working correctly:

```bash
cargo test
```

## License

This project is licensed under the MIT License.
