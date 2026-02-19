# Docker & Kubernetes Guide

This project is designed to be cloud-native, with full support for Docker and Kubernetes.

## 🐳 Docker

The `Dockerfile` uses a multi-stage build process to ensure the final image is minimal and secure.

### Build Stages
1.  **Chef**: Computes the recipe (dependencies) for caching.
2.  **Planner**: Prepares the build plan.
3.  **Builder**: Compiles the actual application.
4.  **Runtime**: A minimal `debian:bookworm-slim` image that contains *only* the compiled binary.

### Commands

**Build the image:**
```bash
docker build -t auth-service .
```

**Run the container:**
```bash
docker run -p 8080:8080 -p 50051:50051 --env-file .env auth-service
```

## 🐙 Docker Compose

To run the service along with its dependencies (e.g., PostgreSQL, though currently self-contained for demo):

```bash
docker-compose up --build
```
This will start the service on ports `8080` (HTTP) and `50051` (gRPC).

## ☸️ Kubernetes

The `k8.yaml` file contains the manifests for deploying to a K8s cluster.

### Resources
*   **Deployment**: Manages the pods, replicas, and updates.
*   **Service**: Exposes the pods (LoadBalancer/NodePort).
*   **Secret**: Securely injects environment variables.

### Deploying

1.  **Create Secrets** (from your local .env):
    ```bash
    kubectl create secret generic auth-service-secrets --from-env-file=.env
    ```

2.  **Apply Configuration**:
    ```bash
    kubectl apply -f k8.yaml
    ```

3.  **Verify**:
    ```bash
    kubectl get pods
    kubectl logs -f deployment/auth-service
    ```
