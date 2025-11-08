use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;

const LOKI_ENDPOINT: &str = "http://grafana-loki-gateway/loki/api/v1/push";

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

    let stream = LokiStream {
        stream: labels,
        values: vec![(timestamp, message.to_string())],
    };

    let request = LokiRequest {
        streams: vec![stream],
    };

    client.post(LOKI_ENDPOINT).json(&request).send().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting log spammer...");

    // Calculate delay for roughly 500 requests per minute
    let delay = std::time::Duration::from_millis(120); // 120ms = ~500 requests/minute

    loop {
        let message = format!("Info: Log entry at {}", Utc::now());
        send_log(&message).await?;
        println!("Log sent: {}", message);
        tokio::time::sleep(delay).await;
    }
}
