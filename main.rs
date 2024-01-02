// $t@$h
use reqwest;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = "";
    let api_url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=";
    let full_url = format!("{}{}", api_url, api_key);

    let request_body = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": "What is you purpose?"
                    }
                ]
            }
        ]
    });

    match post_to_gemini(&full_url, request_body).await {
        Ok(response) => println!("{:#?}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn post_to_gemini(url: &str, body: serde_json::Value) -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.post(url)
                         .json(&body)
                         .send()
                         .await?
                         .json::<serde_json::Value>()
                         .await?;

    Ok(response)
}
