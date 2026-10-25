
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use ai_agent::{constant::DEEPSEEK_FLASH_MODEL, llm::*, tools::{build_toolbox}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载环境变量
    dotenvy::dotenv()?;

    // 2. 配置 tracing tracing 配置
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // let tools = tools();
    
        let toolbox = build_toolbox();

        let system_prompt = concat!(
            "你是一个全能的助手。",
            "今天是2026年10月25日。",
            "你可以使用工具来搜索最新信息。",
            "重要:当工具返回搜索结果时，",
            "你必须要直接使用这些结果来回答，",
            "不要说我不知道。你的数据可能已经过时，",
        );

        let result = chat_complete(
            DEEPSEEK_FLASH_MODEL,
            Some(system_prompt),
            Some("厦门市最新关于翔安机场的新闻,只要一条数据"),
            &toolbox).await?;
        tracing::info!("最终结果: {:#?}", result);

        let result = chat_complete(
            DEEPSEEK_FLASH_MODEL,
            Some(system_prompt),
            Some("计算123456乘以654321等于多少"),
            &toolbox).await?;
        tracing::info!("最终结果: {:#?}", result);

    Ok(())
}


