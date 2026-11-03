use ai_agent::{constant::DEEPSEEK_FLASH_MODEL, llm::*};
use futures::StreamExt;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let s = chat_stream(
        DEEPSEEK_FLASH_MODEL,
        Some("你是一个全能的助手"),
        Some("道德经的第四章是什么"),
    );

    // 4. 固定流，确保在循环中使用时不会被移动
    futures::pin_mut!(s);
    let mut output = String::new();

    while let Some(result) = s.next().await {
        match result {
            Ok(txt) => {
                output.push_str(&txt);
                print!("{}", txt);
            }
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                return Err(e);
            }
        }
    }

    println!("\n\n🌺output: {:#?}\n", output);
    println!("----------------end----------------");
    // tracing::info!("response: {:#?}", response);

    Ok(())
}
