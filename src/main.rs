use clap::{Parser, Subcommand};
use keyring::Entry;
use reqwest;
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use tokio;

const SERVICE_NAME: &str = "lumen";

#[derive(Parser)]
#[command(name = "lumen")]
#[command(about = "A CLI wrapper for AI interactions", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, env = "API_KEY", hide_env_values = true)]
    api_key: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure API key
    Configure {
        /// Set the API key
        #[arg(short, long)]
        api_key: String,
    },
    /// Generate a text completion
    Complete {
        /// The prompt to complete
        #[arg(short, long)]
        prompt: String,
    },
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

fn get_api_key() -> Result<String, Box<dyn Error>> {
    let entry = Entry::new(SERVICE_NAME, "default")?;
    match entry.get_password() {
        Ok(key) => Ok(key),
        Err(_) => Err(
            "API key not found. Please configure it using 'lumen configure --api-key YOUR_KEY'"
                .into(),
        ),
    }
}

fn save_api_key(key: &str) -> Result<(), Box<dyn Error>> {
    let entry = Entry::new(SERVICE_NAME, "default")?;
    entry.set_password(key)?;
    println!("API key saved successfully!");
    Ok(())
}

async fn make_api_request(
    client: &reqwest::Client,
    api_key: &str,
    payload: serde_json::Value,
) -> Result<String, Box<dyn Error>> {
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    let openai_response: OpenAIResponse = response.json().await?;
    Ok(openai_response
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone())
        .unwrap_or_default())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.command {
        Commands::Configure { api_key } => {
            save_api_key(&api_key)?;
        }
        Commands::Complete { prompt } => {
            let api_key = cli.api_key.unwrap_or_else(|| get_api_key().unwrap());
            let payload = json!({
                "model": "gpt-4o-mini",
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a helpful assistant."
                    },
                    {
                        "role": "user",
                        "content": prompt,
                    }
                ]
            });

            let res = make_api_request(&client, &api_key, payload).await?;
            println!("{}", res);
        }
    }

    Ok(())
}
