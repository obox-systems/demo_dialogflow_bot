use std::io;
use serde_json::Value;
use reqwest::Client;


const TOKEN : &str = "ya29.a0Ad52N39vOrR7d3AHMSCURZhwydZ6daMCDnt7q5ENy2XnZ5RJpM6FtqQIkJMoHLm2AJ5P0bHstYDM1SwdE1PlgrO40XEaP8rHL38ePoPLj0NPVKhxNiU2mRuVPY3N4U9FJ_D9nsRydBwvuRRca80Y1RU5h_EVjPkjEgaCgYKAZgSARISFQHGX2MiRyvNehKI_lPVvN-A6bf3fQ0169";
const PROJECT_ID : &str = "radiant-firefly-420408";
const AGENT_ID : &str = "0138e8f3-dc64-4b49-a253-1204c6c6263c";
const REGION_ID : &str = "global";
const SESSION_ID : &str = "test-session-123";



#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error > > 
{
  
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
















