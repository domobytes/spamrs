# AI Agent Instructions for spamrs

## Project Overview
Spamrs is a Rust-based logging service that forwards logs to Grafana Loki. It's designed to run as a containerized service in Kubernetes, with emphasis on being lightweight and efficient.

## Key Components

### Logging Client (`src/main.rs`)
- Uses Loki's HTTP API to push logs
- Implements Loki's JSON push format with streams and labels
- Automatically adds timestamps and standard labels
- Uses async/await patterns with Tokio runtime
- Make the request in an infinite loop at a rate of roughly 500 times per minutes

## Architecture Decisions
1. **Direct Loki Integration**: Uses Loki's push API endpoint directly instead of going through logging libraries
2. **Container-First Design**: Optimized for Kubernetes deployment with minimal image size
3. **In-Cluster Service Discovery**: Assumes Loki gateway is accessible via service name `grafana-loki-gateway`

## Development Workflows

### Building Locally
```bash
cargo build --release
```

### Building Container
```bash
# Build for AMD64 architecture and push to local registry
docker buildx build --platform linux/amd64 --load -t registry:5000/spamrs:latest .
docker push registry:5000/spamrs:latest
```

### Key Files
- `src/main.rs`: Core logging implementation
- `Dockerfile`: Multi-stage build for minimal image size
- `k8s-deployment.yaml`: Kubernetes deployment configuration
- `Cargo.toml`: Dependencies and project configuration

## Integration Points

### Loki Integration
- Endpoint: `http://grafana-loki-gateway/loki/api/v1/push`
- Content-Type: `application/json`
- Required Labels:
  - `app: "spamrs"`
  - `environment: "production"`

### Resource Requirements
Specified in `k8s-deployment.yaml`:
```yaml
resources:
  requests:
    cpu: "100m"
    memory: "64Mi"
  limits:
    cpu: "200m"
    memory: "128Mi"
```

## Common Operations

### Adding New Labels
Modify the `send_log` function in `src/main.rs`:
```rust
let mut labels = HashMap::new();
labels.insert("app".to_string(), "spamrs".to_string());
labels.insert("environment".to_string(), "production".to_string());
// Add new labels here
```

### Modifying Log Format
The log format is defined in the `main` function. Modify the `format!` macro to change log content:
```rust
let message = format!("Info: Application started at {}", Utc::now());
```