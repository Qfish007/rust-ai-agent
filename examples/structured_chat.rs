use ai_agent::{constant::DEEPSEEK_FLASH_MODEL, llm::*};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let response = chat_complete_structured(
        DEEPSEEK_FLASH_MODEL,
        Some("你是一个全能的助手"),
        Some("我要去北京,如何安排"),
    )
    .await?;

    tracing::info!("response: {:#?}", response);

    Ok(())
}
