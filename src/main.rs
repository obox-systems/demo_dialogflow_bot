use base64::{engine::general_purpose::STANDARD, Engine as _};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, Read};

const ENCODING: &str = "AUDIO_ENCODING_FLAC";
const RATE_HERTZ: i32 = 48000;
const LANGUAGE_CODE: &str = "en";

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(rename = "queryResult")]
    query_result: QueryResult,
}

#[derive(Debug, Deserialize)]
struct QueryResult {
    #[serde(rename = "responseMessages")]
    response_messages: Vec<ResponseMessage>,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    text: Text,
}

#[derive(Debug, Deserialize)]
struct Text {
    text: Vec<String>,
}

/// Newtype struct for code( google api )
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token(pub String);

struct APIClient {
    id: String,
    secret: String,
    acces_token: Token,
    oauth2_host: String,
    project_id: String,
    agent_id: String,
    region_id: String,
    session_id: String,
    client: Client,
}

impl APIClient {
    fn new() -> APIClient {
        APIClient {
            id: "567961860776-44sbr0kdkkimn8u0hg1a2bl0bbr6d1ct.apps.googleusercontent.com"
                .to_string(),
            secret: "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".to_string(),
            acces_token: Token(String::new()),
            oauth2_host: "https://oauth2.googleapis.com".to_string(),
            project_id: "radiant-firefly-420408".to_string(),
            agent_id: "0138e8f3-dc64-4b49-a253-1204c6c6263c".to_string(),
            region_id: "global".to_string(),
            session_id: "test-session-123".to_string(),
            client: Client::new(),
        }
    }

    // Construct the URL for the Dialogflow API endpoint
    fn detect_intent_endpoint(&self) -> String {
        format!( "https://{}-dialogflow.googleapis.com/v3/projects/{}/locations/{}/agents/{}/sessions/{}:detectIntent", self.region_id, self.project_id, self.region_id, self.agent_id, self.session_id )
    }

    /// This function refreshes the access token using the provided refresh token.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID used for authentication.
    /// * `secret` - The client secret used for authentication.
    /// * `refresh_token` - The refresh token obtained during the initial authorization process.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Token` struct representing the refreshed access token, or an error if the refresh operation fails.
    ///
    async fn refresh_access_token(
        &mut self,
        refresh_token: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize, Debug)]
        struct Response {
            access_token: String,
        }

        // Construct the query string for the token refresh request.
        let request_query = format!(
            "&client_id={}&client_secret={}&scope=&refresh_token={}&grant_type=refresh_token",
            self.id, self.secret, refresh_token
        );

        // Send a POST request to the OAuth2 token endpoint to refresh the token.
        let response = reqwest::Client::new()
            .post(format!("{}/token?{}", self.oauth2_host, request_query))
            .header("Content-Length", 0)
            .send()
            .await?;

        // Deserialize the JSON response into the Response struct.
        let body = response.json::<Response>().await?;

        // Update the access token.
        self.acces_token = Token(body.access_token);
        // Create client settings with the predefined Authorization header
        self.client = reqwest::Client::builder()
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    header::AUTHORIZATION,
                    header::HeaderValue::from_str(&format!("Bearer {}", self.acces_token.0))
                        .unwrap(),
                );
                headers
            })
            .build()?;

        Ok(())
    }

    async fn send_voice(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // ### Example from AUDIO "Who are you?"

        let mut file = std::fs::File::open(path)?;
        let mut audio_data = Vec::new();
        file.read_to_end(&mut audio_data)?;

        // Encode the audio data
        let encoded_audio_string = STANDARD.encode(audio_data);

        // Construct the request body for audio input
        let body = json!({
            "queryInput": {
                "audio": {
                    "audio": encoded_audio_string,
                    "config": {
                        "audioEncoding": ENCODING,
                        "sampleRateHertz": RATE_HERTZ
                    }
                },
                "languageCode": LANGUAGE_CODE
            },
            "queryParams": {
                "timeZone": "America/Los_Angeles"
            }
        });

        // Send a POST request to Dialogflow API endpoint

        let response = self
            .client
            .post(self.detect_intent_endpoint())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        // Handle the response
        if response.status().is_success() {
            let response_body = response.text().await?;
            // Deserialize the response
            let response: Response = serde_json::from_str(&response_body)?;

            for message in &response.query_result.response_messages {
                for text in &message.text.text {
                    println!("Send from audio \"Who are you?\"");
                    println!("Response -> {}", text);
                }
            }
        } else {
            println!("Request failed with status code: {}", response.status());
        }

        Ok(())
    }

    async fn send_text(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        //let text = " From an economic point of view, what types of activities does the term business cover? ";

        // Construct the request body for text input
        let body = json!({
            "queryInput": {
                "text": {
                    "text": text
                },
                "languageCode": "en"
            },
            "queryParams": {
                "timeZone": "America/Los_Angeles"
            }
        });

        // Send a POST request to Dialogflow API endpoint for text input
        let response = self
            .client
            .post(self.detect_intent_endpoint())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        // Handle the response
        if response.status().is_success() {
            let response_body = response.text().await?;
            // Deserialize the response
            let response: Response = serde_json::from_str(&response_body)?;

            // Text processing
            for message in &response.query_result.response_messages {
                for text in &message.text.text {
                    println!("Response -> {}", text);
                }
            }
        } else {
            println!("Request failed with status code: {}", response.status());
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut new_client = APIClient::new();
    new_client
        .refresh_access_token("1//09bTZybaA1m24CgYIARAAGAkSNwF-L9IrYIYLsjBcmUZZ8JgOHaDNSiSc9c5wLiGMDt8ICtn_UJ3bmCex0X03cX7kGwTy5fnmrak")
        .await?;

    loop {
        println!("Select an option:");
        println!("1. Send voice input");
        println!("2. Send text input");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                println!("Enter the path to the audio file:");
                let mut path = String::new();
                io::stdin().read_line(&mut path)?;
                let path = path.trim();
                new_client.send_voice(path).await?;
            }
            "2" => {
                println!("Enter your question:");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let text = input.trim();
                new_client.send_text(text).await?;
            }
            "3" => break,
            _ => println!("Invalid option! Please choose again."),
        }
    }

    Ok(())
}
