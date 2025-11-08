# spamrs

A lightweight Rust-based logging service that forwards logs to Grafana Loki. Designed to run in Kubernetes and generate test log data at a configurable rate.

## Overview

Spamrs is a log generator service that:
- Sends logs to Grafana Loki using its HTTP API
- Runs in Kubernetes alongside Loki
- Generates approximately 500 logs per minute
- Uses standard labels for log identification and filtering
- Provides timestamps in nanosecond precision

## Prerequisites

- Rust (latest stable or nightly)
- Docker
- Kubernetes cluster with Grafana Loki installed
- The Loki gateway service should be accessible at `grafana-loki-gateway`

## Development

### Building & running locally

```bash
# Build the project
cargo build --release

# Run locally (requires LOKI endpoint to be accessible). Note: the binary
# no longer prints log activity to stdout; logs are sent directly to Loki.
# If you want to run with an alternate Loki endpoint use the LOKI_ENDPOINT env var.
LOKI_ENDPOINT="http://grafana-loki-gateway.grafana/loki/api/v1/push" cargo run --release
```

### Building and Pushing Container

```bash

# Build for AMD64 architecture and load into local Docker
docker buildx build --platform linux/amd64 --load -t registry:5000/spamrs:latest .

# Push to local registry
docker push registry:5000/spamrs:latest
```

### Deploying to Kubernetes

The example `k8s-deployment.yaml` provided in this repo is already configured to:
- Reference the image `registry:5000/spamrs:latest` (update to your registry as needed)
- Populate `HOSTNAME` and `K8S_NAMESPACE_NAME` using the Downward API so the pod name and namespace are added as labels to each stream

To apply the deployment:

```bash
# Apply the deployment configuration (example: deploy into the `grafana` namespace)
kubectl apply -n grafana -f k8s-deployment.yaml
```

If you need to override the Loki endpoint in-cluster, set `LOKI_ENDPOINT` in the pod spec's env (or a ConfigMap):

```yaml
env:
  - name: LOKI_ENDPOINT
    value: "http://grafana-loki-gateway.grafana/loki/api/v1/push"
  - name: HOSTNAME
    valueFrom:
      fieldRef:
        fieldPath: metadata.name
  - name: K8S_NAMESPACE_NAME
    valueFrom:
      fieldRef:
        fieldPath: metadata.namespace
```

## Configuration

### Log Labels

The service adds the following default labels to all logs:
- `app: "spamrs"`
- `environment: "production"`

### Resource Limits

The Kubernetes deployment is configured with the following resource limits:
```yaml
resources:
  requests:
    cpu: "100m"
    memory: "64Mi"
  limits:
    cpu: "200m"
    memory: "128Mi"
```

## Log Format

Each log entry includes:
- ISO 8601 timestamp
- Standard labels
- Log message with timestamp

Example log message:
```
Info: Log entry at 2025-11-07T12:34:56.789012345Z
```

## Project Structure

- `src/main.rs` - Core logging implementation
- `Dockerfile` - Multi-stage build configuration
- `Dockerfile` - Multi-stage build configuration. The Dockerfile uses a dependency-caching pattern: it pre-copies `Cargo.toml`/`Cargo.lock` and does a dummy build to cache Rust dependency compilation in an earlier layer; this speeds up iterative builds.
- `k8s-deployment.yaml` - Kubernetes deployment configuration
- `Cargo.toml` - Rust project and dependency configuration