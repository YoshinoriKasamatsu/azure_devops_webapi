use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;
use std::error::Error;

pub(crate) async fn request_post(pat: &str, url: &String, query: String) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.post(url)
        .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!(":{}",
                                                                                              pat))))
        .header("Content-Type", "application/json")
        .body(query)
        .send()
        .await?;

    let body = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    Ok(json)
}

pub(crate) async fn request_get(pat: &str, url: &String) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url)
        .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!(":{}",
                                                                                              pat))))
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await?;

    // ステータスコードの確認
    if !response.status().is_success() {
        println!("Error: {}", response.status());
        return Ok(serde_json::Value::Null);
    }
    let body: String = response.text().await?;
    
    let json: serde_json::Value = serde_json::from_str(&body)?;
    Ok(json)
}