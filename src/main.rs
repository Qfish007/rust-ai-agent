
// use async_openai::types::chat::ChatCompletionModerationInput::Results;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use ai_agent::{llm::*, constant::DEEPSEEK_FLASH_MODEL};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let response = chat_complete_structured(
        DEEPSEEK_FLASH_MODEL,
        Some("你是一个全能的助手"),
        Some("中国的首都在哪里,计划旅行，如何安排"),
    )
    .await?;

    tracing::info!("response: {:#?}", response);

    

    Ok(())
}
