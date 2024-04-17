# demo_dialogflow_bot

### Abstract


This program interacts with the Dialogflow API to process voice and text inputs and receive corresponding responses from the chatbot.
A chatbot is a trained special NLP model using Vertex AI.
Bot trained on this small data (just for example)
[IJRAR23C3149Published.pdf](https://github.com/obox-systems/demo_dialogflow_bot/files/14983657/IJRAR23C3149Published.pdf)


### Program Details

- #### APIClient:

The APIClient struct encapsulates the functionality for interacting with Google services.
It handles authentication, token refreshing, and sending requests to the Dialogflow API.
- #### Authentication:

The `refresh_access_token()` method refreshes the access token using the provided refresh token.
It constructs a request to the OAuth2 token endpoint and updates the access token for subsequent requests.
- #### Sending Input:

The `send_voice()` method sends audio input to Dialogflow.
It encodes the audio data into a Base64 string, constructs the request body, and sends a POST request to the Dialogflow API endpoint.
The `send_text()` method sends text input to Dialogflow.
It constructs the request body for text input and sends a POST request to the Dialogflow API endpoint.
- #### Main Functionality:

The `main()` function serves as the entry point for the program.
It displays a menu for user interaction, allowing them to choose between voice and text input options.
Based on the user's selection, it prompts for input, sends the request to Dialogflow, and displays the responses.

### Competencies

For this demo, I've used `serde`/`serde_json` crates for API messages serialization and deserialization,
the `base64` crate for encoding audio, the `reqwest` crate for sending requests to the API
and `tokio` for async runtime.

### Try it out!

1. Install [Rust](https://rustup.rs/)
2. Run the app

```bash
cargo run --release
```

3. Select an Option:

Upon running, the program will display a menu with options:
Option 1: Send voice input.
Option 2: Send text input.
Option 3: Exit the program.

4. Provide Input:

Choose an option by entering the corresponding number.
Follow the prompts to provide either a path to an audio file (for voice input) or enter your question (for text input).

5. View Responses:

After providing input, the program will send the request to Dialogflow.
It will display the responses received from Dialogflow.

### Screenshot or gif

![Снимок экрана 2024-04-15 230035](https://github.com/obox-systems/demo_dialogflow_bot/assets/104863923/20761561-acde-43be-8fe3-169c178ca879)
