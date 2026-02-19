# Stage 1: Builder
# Use the official Rust image to build the application
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Install protobuf compiler
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

# Copy the Cargo configuration files first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to build dependencies
# This prevents re-downloading/building crates when only source code changes
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# Build the dependencies (release mode)
RUN cargo build --release

# Remove the dummy source and build artifacts for the main binary
RUN rm -f target/release/deps/auth_service*
RUN rm src/main.rs

# Copy the actual source code
COPY . .

# Build the actual application
# We use touch to modify the timestamp of the main file to force a rebuild
RUN touch src/main.rs && cargo build --release

# Stage 2: Runtime
# Use a minimal Linux image for the final container
FROM debian:bookworm-slim

# Install OpenSSL and CA certificates (needed for many Rust apps making HTTPS requests)
RUN apt-get update && apt-get install -y \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
# Note: Adjust the binary name if your Cargo.toml package name is different
COPY --from=builder /usr/src/app/target/release/auth-service /usr/local/bin/auth-service

# Set the binary as the entrypoint
CMD ["auth-service"]
