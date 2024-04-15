use std::io::{self, Read};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;
use base64::{engine::general_purpose::STANDARD, Engine as _};


const PROJECT_ID : &str = "radiant-firefly-420408";
const AGENT_ID : &str = "0138e8f3-dc64-4b49-a253-1204c6c6263c";
const REGION_ID : &str = "global";
const SESSION_ID : &str = "test-session-123";

const ENCODING : &str = "AUDIO_ENCODING_FLAC";
const RATE_HERTZ : i32 = 48000;
const LANGUAGE_CODE : &str = "en";

const OAUTH2_HOST : &str = "https://oauth2.googleapis.com";
const REFRESH_TOKEN : &str = "1//09bTZybaA1m24CgYIARAAGAkSNwF-L9IrYIYLsjBcmUZZ8JgOHaDNSiSc9c5wLiGMDt8ICtn_UJ3bmCex0X03cX7kGwTy5fnmrak";
const CLIENT_ID : &str = "567961860776-44sbr0kdkkimn8u0hg1a2bl0bbr6d1ct.apps.googleusercontent.com";
const CLIENT_SECRET : &str = "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";



/// Newtype struct for code( google api )
#[ derive( Debug, Clone, Deserialize, Serialize ) ]
pub struct Token( pub String );


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
pub async fn refresh_access_token( client_id : &str, secret : &str, refresh_token : &str )
-> Result< Token, Box< dyn std::error::Error > > 
{

  #[ derive( Deserialize, Debug ) ]
  struct Response
  {
    access_token : String
  }

  // Construct the query string for the token refresh request.
  let request_query = format!
  (
    "&client_id={}&client_secret={}&scope=&refresh_token={}&grant_type=refresh_token",
    client_id, secret, refresh_token
  );

  // Send a POST request to the OAuth2 token endpoint to refresh the token.
  let response = reqwest::Client::new()
  .post( format!( "{}/token?{}", OAUTH2_HOST, request_query ) )
  .header("Content-Length", 0 )
  .send()
  .await?;

  // Deserialize the JSON response into the Response struct.
  let body = response
  .json::< Response >()
  .await?;

  // Return the refreshed access token.
  Ok( Token( body.access_token ) )
}











#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error > > 
{

  // ### Example from AUDIO "Who are you?"


  // Specify the path to the audio file
  let audio_file_name = "../for_bot.flac";
  let mut file = std::fs::File::open( audio_file_name )?;
  let mut audio_data = Vec::new();
  file.read_to_end( &mut audio_data )?;

  // Encode the audio data
  let encoded_audio_string = STANDARD.encode( audio_data );
 
  

  // Construct the request body for audio input
  let body = format!
  (
    r#"
    {{
      "queryInput": 
      {{
        "audio": 
        {{
          "audio": "{encoded_audio_string}",
          "config": 
          {{
            "audioEncoding": "{ENCODING}",
            "sampleRateHertz": {RATE_HERTZ}
          }}
        }},
        "languageCode": "{LANGUAGE_CODE}"
      }},
    }}
    "#
  );

  // Construct the URL for the Dialogflow API endpoint
  let url = format!( "https://{REGION_ID}-dialogflow.googleapis.com/v3/projects/{PROJECT_ID}/locations/{REGION_ID}/agents/{AGENT_ID}/sessions/{SESSION_ID}:detectIntent" );
  
  // Refresh the access token
  let acces_token = refresh_access_token( CLIENT_ID, CLIENT_SECRET, REFRESH_TOKEN ).await?;
  
  // Send a POST request to Dialogflow API endpoint
  let client = Client::new();
  let response = client
  .post( url )
  .header( "Content-Type", "application/json" )
  .header( "Authorization", format!( "Bearer {}", acces_token.0 ) )
  .body( body )
  .send()
  .await?;

  // Handle the response
  if response.status().is_success() 
  {
    let response_body = response.text().await?;
    let value : Value = serde_json::from_str( &response_body )?;
    if let Some( messages ) = value[ "queryResult" ][ "responseMessages" ].as_array() 
    {
      for message in messages 
      {
        if let Some( texts ) = message[ "text" ][ "text" ].as_array() 
        {
          for text in texts 
          {
            if let Some( text_string ) = text.as_str() 
            {
              println!( "Response -> {}", text_string );
            }
          }
        }
      }
    }
  } 
  else 
  {
    println!( "Request failed with status code: {}", response.status() );
  }



  // ### from text
  loop 
  {
    let mut input = String::new();

    println!( "Enter your question: " );
    io::stdin().read_line( &mut input )?;

    let text = input.trim();
    if text.to_lowercase() == "exit" 
    {
      break; 
    }

    //let text = " From an economic point of view, the term What types of activities does the term business cover? ";

    // Construct the request body for text input
    let body = format!
    (
      r#"
      {{
        "queryInput": {{
          "text": {{
            "text": "{text}"
          }},
          "languageCode": "en"
        }},
        "queryParams": {{
          "timeZone": "America/Los_Angeles"
        }}
      }}
      "#
    );


    let url = format!( "https://{REGION_ID}-dialogflow.googleapis.com/v3/projects/{PROJECT_ID}/locations/{REGION_ID}/agents/{AGENT_ID}/sessions/{SESSION_ID}:detectIntent" );


    let acces_token = refresh_access_token( CLIENT_ID, CLIENT_SECRET, REFRESH_TOKEN ).await?;

    // Send a POST request to Dialogflow API endpoint for text input
    let client = Client::new();
    let response = client
    .post( url )
    .header( "Content-Type", "application/json" )
    .header( "x-goog-user-project", PROJECT_ID )
    .header( "Authorization", format!( "Bearer {}", acces_token.0 ) )
    .body( body )
    .send()
    .await?;

    // Handle the response
    if response.status().is_success() 
    {
      let response_body = response.text().await?;
      let value : Value = serde_json::from_str( &response_body )?;
      if let Some( messages ) = value[ "queryResult" ][ "responseMessages" ].as_array() 
      {
        for message in messages 
        {
          if let Some( texts ) = message[ "text" ][ "text" ].as_array() 
          {
            for text in texts 
            {
              if let Some( text_string ) = text.as_str() 
              {
                println!( "Response -> {}", text_string );
              }
            }
          }
        }
      }
    } 
    else 
    {
      println!( "Request failed with status code: {}", response.status() );
    }
  }
  

  Ok( () )
}
















