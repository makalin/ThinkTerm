use reqwest::Client;
use serde_json::json;
use std::collections::VecDeque;

const OPENAI_API_KEY: &str = "your_openai_api_key"; // Replace with your key

pub async fn ask_ai(history: &VecDeque<String>, prompt: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let context: Vec<_> = history.iter().map(|entry| json!({"role": "user", "content": entry})).collect();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", OPENAI_API_KEY))
        .json(&json!({
            "model": "gpt-4", 
            "messages": context,
        }))
        .send()
        .await?;
    
    let json_response: serde_json::Value = response.json().await?;
    Ok(json_response["choices"][0]["message"]["content"].as_str().unwrap_or("No response").to_string())
}
