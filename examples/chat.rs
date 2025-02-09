extern crate transformrs;

use transformrs::openai;
use transformrs::Message;
use transformrs::Provider;

#[tokio::main]
async fn main() {
    let messages = vec![
        Message::from_str("system", "You are a helpful assistant."),
        Message::from_str("user", "This is a test. Please respond with 'hello world'."),
    ];
    let keys = transformrs::load_keys(".env");
    let provider = Provider::DeepInfra;
    let key = keys.for_provider(&provider).unwrap();
    let model = "meta-llama/Llama-3.3-70B-Instruct";
    // Using the OpenAI-compatible API.
    let resp = openai::chat_completion(&provider, &key, model, &messages)
        .await
        .unwrap()
        .structured()
        .unwrap();
    println!("{}", resp.choices[0].message.content);
}

// output:
// hello world