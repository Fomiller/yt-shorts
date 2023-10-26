use ::std::time::Duration;
use chatgpt::config::*;
use chatgpt::prelude::*;
use chatgpt::types::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let mut prompt: String =
        "Write me a text message that tells my girl friend to drive safe.".to_string();
    if args.len() > 1 {
        prompt = args[1].clone();
    }
    let key = env::var("CHATGPT_API_KEY")?;

    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            .timeout(Duration::new(30, 0))
            .build()
            .unwrap(),
    )?;

    let response: CompletionResponse = client.send_message(prompt).await?;

    println!("Response: \n{}\n", response.message().content);
    Ok(())
}
