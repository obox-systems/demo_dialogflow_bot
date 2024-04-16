# demo_dialogflow_bot

### Abstract


This program interacts with the Dialogflow API to process voice and text inputs and receive corresponding responses from the chatbot.
A chatbot is a trained special NLP model using Vertex AI.
Bot trained on this small data (just for example)
[IJRAR23C3149Published.pdf](https://github.com/obox-systems/demo_dialogflow_bot/files/14983657/IJRAR23C3149Published.pdf)

Voice Input: The program reads audio data from a file, encodes it into a Base64 string, constructs a JSON request body with the audio data, and sends a POST request to the Dialogflow API endpoint. Upon receiving a successful response, it parses the response JSON and extracts the text responses, printing them to the console.

Text Input: The program prompts the user to input a question or text. It constructs a JSON request body with the text input, sends a POST request to the Dialogflow API endpoint, and handles the response similarly to the voice input, parsing the response JSON and printing the text responses to the console.

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

### Screenshot or gif
![Снимок экрана 2024-04-15 230035](https://github.com/obox-systems/demo_dialogflow_bot/assets/104863923/20761561-acde-43be-8fe3-169c178ca879)


