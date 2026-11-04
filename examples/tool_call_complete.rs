use ai_agent::{constant::DEEPSEEK_FLASH_MODEL, llm::*, tools::*};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载环境变量
    dotenvy::dotenv()?;

    // 2. 配置 tracing tracing 配置
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let toolbox = tools();

    let result = chat_complete(
        DEEPSEEK_FLASH_MODEL,
        Some(
            r#"你是一个全能的助手。
        今天是2026年10月25日。
        你可以使用工具来搜索最新信息。
        重要:当工具返回搜索结果时，
        你必须要直接使用这些结果来回答，
        不要说我不知道。你的数据可能已经过时，
        请优先使用信任工具返回的内容。"#,
        ),
        Some("今天厦门最热的新闻"),
        toolbox,
    ).await?;
    tracing::info!("最终结果: {:#?}", result);

    Ok(())
}
