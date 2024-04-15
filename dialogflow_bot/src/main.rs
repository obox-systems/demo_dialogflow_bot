use std::io::{self, Read};
use serde_json::Value;
use reqwest::Client;
use base64::{engine::general_purpose::STANDARD, Engine as _};

const TOKEN : &str = "ya29.a0Ad52N38k2ODg0Gk8iy6hAf1L95yJTdP9YZbJZ016KAIBNH_tnCFbDu1Gh5e8Zgh3gvp_4oRQjFOrpnDws6tN90bTiVX6cPI_tteZPOWvYP5hJDy5mcT_mKmLHe4NHNAlKJ_LgWLuBx6_A5zRwfkT8LQ_XxCDCws8ZD8aCgYKAXYSARISFQHGX2Mi4Ip2UDCe4SuA584ByQmrXg0170";
const PROJECT_ID : &str = "radiant-firefly-420408";
const AGENT_ID : &str = "0138e8f3-dc64-4b49-a253-1204c6c6263c";
const REGION_ID : &str = "global";
const SESSION_ID : &str = "test-session-123";

const ENCODING : &str = "AUDIO_ENCODING_FLAC";
const RATE_HERTZ : i32 = 48000;
const LANGUAGE_CODE : &str = "en";


#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error > > 
{

  // ### from AUDIO 

  let audio_file_name = "../for_bot.flac";
 
  let mut file = std::fs::File::open( audio_file_name )?;
  let mut audio_data = Vec::new();
  file.read_to_end( &mut audio_data )?;


  let encoded_audio_string = STANDARD.encode( audio_data );
 
  //let text = " From an economic point of view, the term What types of activities does the term business cover? ";
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


  let url = format!( "https://{REGION_ID}-dialogflow.googleapis.com/v3/projects/{PROJECT_ID}/locations/{REGION_ID}/agents/{AGENT_ID}/sessions/{SESSION_ID}:detectIntent" );

  let client = Client::new();
  let response = client
  .post( url )
  .header( "Content-Type", "application/json" )
  .header( "Authorization", format!( "Bearer {TOKEN}" ) )
  .body( body )
  .send()
  .await?;

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
    println!( "Request failed with status code: {:?}", response );
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

    let client = Client::new();
    let response = client
    .post( url )
    .header( "Content-Type", "application/json" )
    .header( "x-goog-user-project", PROJECT_ID )
    .header( "Authorization", format!( "Bearer {TOKEN}" ) )
    .body( body )
    .send()
    .await?;

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
















