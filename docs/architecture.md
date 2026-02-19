# Project Architecture & Structure

The **Auth-Service** follows a **Clean Architecture** (or Hexagonal/Onion-inspired) pattern. The goal is to separate the core business logic from the external interfaces (HTTP, gRPC, Database).

## High-Level Layers

1.  **Transport Layer** (Entry Points)
    *   **HTTP (REST)**: Handled by Actix Web (`src/handlers`, `src/routes`).
    *   **gRPC**: Handled by Tonic (`src/grpc`).
2.  **Service Layer** (`src/services`): Contains the core business logic. It orchestrates data flow and rules, agnostic of how the request arrived (HTTP or gRPC).
3.  **Data Layer** (`src/repository`, `src/db`): Handles direct interaction with the database (PostgreSQL via Diesel).
4.  **Domain/Model Layer** (`src/models`, `src/dto`): Defines the data structures used across the application.

## Directory Breakdown

```
src/
├── config/         # App configuration (Env vars, Logger, Swagger)
├── db/             # Database connection pooling setup
├── dto/            # Data Transfer Objects (API contracts)
├── errors/         # Custom Error types for unified error handling
├── grpc/           # gRPC implementation and generated code
│   ├── implementation/ # Logic for gRPC services
│   └── traits/         # Generated traits from .proto files
├── handlers/       # HTTP Controllers (Parses HTTP -> Calls Service)
├── middleware/     # Cross-cutting concerns (Auth, Logging, RateLimit)
├── models/         # Database entities (Diesel structs)
├── repository/     # CRUD operations
├── routes/         # Actix Web route configurations
├── services/       # Business Logic
└── utils/          # Helpers (Hashing, JWT, Validation)
```

## Data Flow

### HTTP Request
1.  **Request** hits `src/routes`.
2.  **Middleware** runs (Auth check, Logging).
3.  **Handler** (`src/handlers`) receives the request and deserializes it into a DTO (`src/dto`).
4.  **Handler** calls the **Service** (`src/services`).
5.  **Service** performs logic, potentially calling **Repository** (`src/repository`).
6.  **Repository** executes DB query via **DB** pool (`src/db`) returning a **Model** (`src/models`).
7.  **Service** maps Model -> DTO.
8.  **Handler** returns HTTP Response with DTO.

### gRPC Request
1.  **Request** hits `src/main.rs` (Tonic server).
2.  **Implementation** (`src/grpc/implementation`) receives the gRPC request struct.
3.  **Implementation** calls the **Service** (reusing the same logic as HTTP!).
4.  **Service** returns data.
5.  **Implementation** maps data to gRPC response struct and returns.

## Key Design Principles

*   **Dependency Injection**: Passed mainly via function arguments or Actix `Data<T>`.
*   **Separation of Concerns**: Handlers don't know about SQL; Services don't know about HTTP status codes.
*   **Type Safety**: Leveraging Rust's strong type system to prevent errors.
