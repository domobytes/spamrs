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
 - Uses Loki's HTTP API to push logs
 - Implements Loki's JSON push format with streams and labels
 - Automatically adds timestamps and standard labels
 - Uses async/await patterns with Tokio runtime
 - Make the request in an infinite loop at a rate of roughly 500 times per minutes
 - Runs continuously (infinite loop) to generate traffic; rate is ~500 req/min (120ms delay)
 - Stdout is silenced in normal operation: the binary no longer prints per-log messages to stdout. Logs are delivered to Loki via HTTP. Errors still surface via stderr or returned Results.

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

### Dockerfile / Build notes
- The `Dockerfile` uses a multi-stage build. Key points:
  - Builder image: `rustlang/rust:nightly-slim` (nightly used to match toolchain and lockfile compatibility)
  - Installs build-time deps: `pkg-config`, `libssl-dev`
  - Runtime image: `debian:bookworm-slim` with `ca-certificates` and `libssl3`
  - Build caching: the Dockerfile pre-copies `Cargo.toml` and `Cargo.lock` and performs a dummy build step so dependency compilation is cached in an earlier Docker layer; this significantly speeds up iterative builds.
  - If you need reproducible builds on CI, ensure the builder image and cargo version match the local toolchain.

### Key Files
- `src/main.rs`: Core logging implementation
- `Dockerfile`: Multi-stage build for minimal image size
- `k8s-deployment.yaml`: Kubernetes deployment configuration
- `Cargo.toml`: Dependencies and project configuration

## Integration Points

### Loki Integration
- Endpoint (default): `http://grafana-loki-gateway.grafana/loki/api/v1/push` — this can be overridden with the `LOKI_ENDPOINT` environment variable.
- Content-Type: `application/json`
- Required / default Labels (added by `send_log`):
  - `app: "spamrs"`
  - `environment: "production"`
  - `hostname`: sourced from the `HOSTNAME` environment variable when available (Kubernetes sets this to the pod name); falls back to `"unknown"` if not set. Use the Downward API in k8s to explicitly expose pod name if desired. Example snippet:

```yaml
env:
  - name: HOSTNAME
    valueFrom:
      fieldRef:
        fieldPath: metadata.name
```
  - `log_level: "info"` (static for this generator)
  - `k8s_namespace_name`: sourced from the `K8S_NAMESPACE_NAME` environment variable (populated via the Downward API with `metadata.namespace`). Falls back to `"unknown"` if not set.
    - `k8s_namespace_name`: sourced from the `K8S_NAMESPACE_NAME` environment variable (populated via the Downward API with `metadata.namespace`). Falls back to `"unknown"` if not set. Example snippet:

  ```yaml
  env:
    - name: K8S_NAMESPACE_NAME
      valueFrom:
        fieldRef:
          fieldPath: metadata.namespace
  ```

To expose the namespace via Downward API, add to the container env:

```yaml
env:
  - name: K8S_NAMESPACE_NAME
    valueFrom:
      fieldRef:
        fieldPath: metadata.namespace
```

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