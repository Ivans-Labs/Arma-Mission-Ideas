use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const AI_API_URL: &str = "http://localhost:11434/api";

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    context: Vec<i32>,
}

pub async fn generate_text(model: &str, prompt: &str) -> Result<String, Error> {
    let client = Client::new();
    let api_endpoint = format!("{}/generate", AI_API_URL);

    let generate_request = GenerateRequest {
        model: model.to_owned(),
        prompt: prompt.to_owned(),
        stream: false,
    };

    let response = client.post(&api_endpoint)
        .json(&generate_request)
        .send()
        .await?;

    let generate_response: GenerateResponse = response.json().await?;
    Ok(generate_response.response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_text() {
        let model = "llama2";
        let prompt = "What color is the sky at different times of the day? Respond using JSON";

        match generate_text(model, prompt).await {
            Ok(response) => {
                println!("Response received: {}", response);
                assert!(response.contains("morning"));
            },
            Err(e) => panic!("Failed to generate text: {}", e),
        }
    }

}
