use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Add your route handlers here, e.g.:
        // crate::handlers::auth::login,
    ),
    components(
        schemas(
            // Add your schemas here, e.g.:
            // crate::dto::auth::LoginRequest,
        )
    ),
    tags(
        (name = "auth-service", description = "Authentication Service API")
    ),
    servers(
        (url = "/api", description = "Local server"),
    )
)]
pub struct ApiDoc;
