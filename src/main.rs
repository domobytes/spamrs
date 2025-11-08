use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;

// Loki endpoint is configurable via the LOKI_ENDPOINT env var. If not set,
// default to the in-cluster grafana-loki-gateway service.
// Example: export LOKI_ENDPOINT="http://grafana-loki-gateway/loki/api/v1/push"

#[derive(Serialize)]
struct LokiStream {
    stream: HashMap<String, String>,
    values: Vec<(String, String)>,
}

#[derive(Serialize)]
struct LokiRequest {
    streams: Vec<LokiStream>,
}

async fn send_log(message: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or(0).to_string();

    let mut labels = HashMap::new();
    labels.insert("app".to_string(), "spamrs".to_string());
    labels.insert("environment".to_string(), "production".to_string());
    // Add hostname label: prefer HOSTNAME env (set by k8s), fallback to "unknown"
    let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());
    labels.insert("hostname".to_string(), hostname);
    // Add k8s namespace label: prefer K8S_NAMESPACE_NAME env (set by Downward API), fallback to "unknown"
    let k8s_ns = std::env::var("K8S_NAMESPACE_NAME").unwrap_or_else(|_| "unknown".to_string());
    labels.insert("k8s_namespace_name".to_string(), k8s_ns);
    // Static log level for this generator
    labels.insert("log_level".to_string(), "info".to_string());

    let stream = LokiStream {
        stream: labels,
        values: vec![(timestamp, message.to_string())],
    };

    let request = LokiRequest {
        streams: vec![stream],
    };

    let loki_endpoint = std::env::var("LOKI_ENDPOINT")
        .unwrap_or_else(|_| "http://grafana-loki-gateway.grafana/loki/api/v1/push".to_string());

    client.post(&loki_endpoint).json(&request).send().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Calculate delay for roughly 500 requests per minute
    let delay = std::time::Duration::from_millis(120); // 120ms = ~500 requests/minute

    loop {
        let message = format!("Info: Log entry at {}", Utc::now());
        send_log(&message).await?;
        // stdout silenced: logs are sent to Loki via HTTP
        tokio::time::sleep(delay).await;
    }
}
