
// use std::io::Write;


use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use ai_agent::{llm::*, constant::DEEPSEEK_FLASH_MODEL};
use futures::StreamExt;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载环境变量
    dotenvy::dotenv()?;

    // 2. 配置 tracing tracing 配置
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // 3. 调用 chat_stream 函数
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
                // stdout 是行缓冲的，流式片段通常没有换行，
                // 必须手动 flush 才能在终端上实时看到逐字输出
                // std::io::stdout().flush()?;
            },
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                return Err(e);
            },
        }
    }

    println!("output: {:#?}", output);
    println!("-----------------");
    // tracing::info!("response: {:#?}", response);



    Ok(())
}


