# How to Create a New API Endpoint

This guide outlines the steps to add a new API endpoint to the Auth-Service, following the Clean Architecture pattern.

## Overview of Steps

1.  **Define the Request/Response (DTO)**: Create structs for input/output.
2.  **Create the Service Layer**: Implement the business logic.
3.  **Create the Handler Layer**: Handle the HTTP request/response.
4.  **Register the Route**: Map the URL path to the handler.
5.  **Export Modules**: Ensure everything is accessible via `mod.rs`.

---

## Step 1: Define DTOs (`src/dto/`)

Create a new file or add to an existing one in `src/dto/`. Define your request and response structures using `serde`.

**Example:** `src/dto/example.rs`
```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateExampleRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct ExampleResponse {
    pub id: String,
    pub name: String,
    pub message: String,
}
```

## Step 2: Service Layer (`src/services/`)

Create a service to handle the business logic. This keeps your handlers clean.

**Example:** `src/services/example_service.rs`
```rust
use crate::dto::example::{CreateExampleRequest, ExampleResponse};
use uuid::Uuid;

pub async fn create_example(data: CreateExampleRequest) -> Result<ExampleResponse, String> {
    // Perform logic (e.g., DB operations, calculations)
    let response = ExampleResponse {
        id: Uuid::new_v4().to_string(),
        name: data.name,
        message: "Example created successfully".to_string(),
    };
    Ok(response)
}
```

## Step 3: Handler Layer (`src/handlers/`)

Create a handler function that Actix Web will call. This function parses the request, calls the service, and returns an HTTP response.

**Example:** `src/handlers/example.rs`
```rust
use actix_web::{web, HttpResponse, Responder};
use crate::dto::example::CreateExampleRequest;
use crate::services::example_service;

pub async fn perform_example(
    body: web::Json<CreateExampleRequest>, 
) -> impl Responder {
    match example_service::create_example(body.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
```

## Step 4: Route Registration (`src/routes/`)

Define the route path and link it to the handler.

**Example:** `src/routes/example_routes.rs`
```rust
use actix_web::web;
use crate::handlers::example::perform_example;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/example")
            .route(web::post().to(perform_example))
    );
}
```

## Step 5: Wire It Up (`src/main.rs`)

Finally, register your route config in the main application scope in `src/main.rs`.

```rust
// ... inside main ...
.service(
    web::scope("/api")
        .configure(crate::routes::example_routes::scoped_config) // Add this line
        // other routes...
)
```

## Step 6: Documentation (Swagger)

Don't forget to add your new DTOs and paths to `src/config/swagger.rs` so they appear in the Swagger UI!
