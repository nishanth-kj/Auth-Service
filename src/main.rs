use actix_web::{App, HttpServer, web};
use auth_service::config::env::{Environment, get_env};
use auth_service::config::swagger::ApiDoc;
use auth_service::grpc::implementation::test_impl::MyTestService;
use auth_service::grpc::traits::test::test_service_server::TestServiceServer;
use log::{debug, error, info, warn};
use std::net::SocketAddr;
use tonic::transport::Server;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    let env = get_env();

    // Initialize logging based on environment
    let env_str = match env {
        Environment::Development => "DEVELOPMENT",
        Environment::Production => "PRODUCTION",
    };

    if let Err(e) = auth_service::config::logger::setup_logger(env_str) {
        println!("Error setting up logger: {}", e);
    }

    info!("Starting Auth-Service in {:?} mode...", env);
    debug!("Debug logging is enabled");
    warn!("System is starting up - this is a sample warning");
    error!("This is a sample error");

    if env == Environment::Development {
        info!("Swagger UI enabled at /swagger-ui");
    }

    // Define addresses
    let http_addr = "0.0.0.0:8080";
    let grpc_addr = "0.0.0.0:50051".parse::<SocketAddr>().unwrap();

    let env_for_server = env.clone();

    // Start HTTP Server (Actix)
    let http_server = HttpServer::new(move || {
        let mut app = App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route("/", web::get().to(|| async { "Auth Service is running!" }))
            .service(
                web::scope("/api")
                    .route(
                        "/",
                        web::get().to(|| async { "Auth Service is running (API)!" }),
                    )
                    .route("/health", web::get().to(|| async { "OK" })),
            );

        if env_for_server == Environment::Development {
            app = app
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-docs/openapi.json", ApiDoc::openapi()),
                )
                .service(web::redirect("/swagger-ui", "/swagger-ui/"));
        }

        app
    })
    .bind(http_addr)?
    .run();

    info!("HTTP server listening on {}", http_addr);

    // Start gRPC Server (Tonic)
    let grpc_server = Server::builder()
        .add_service(TestServiceServer::new(MyTestService))
        .serve(grpc_addr);

    info!("gRPC server listening on {}", grpc_addr);

    // Run both servers concurrently
    let (http_result, grpc_result) = tokio::join!(http_server, grpc_server);

    if let Err(e) = http_result {
        error!("HTTP server error: {}", e);
    }
    if let Err(e) = grpc_result {
        error!("gRPC server error: {}", e);
    }

    Ok(())
}
