
// use std::io::Write;


use tokio::task::JoinSet;
use tracing::{Instrument, Level};
use tracing_subscriber::FmtSubscriber;
use ai_agent::{llm::*, constant::DEEPSEEK_FLASH_MODEL};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let prompts = vec!["道德经的第四章是什么",
        "道德经的第五章是什么",
        "道德经的第六章是什么",
        "道德经的第七章是什么",
        "道德经的第八章是什么",
        "道德经的第九章是什么",
        "道德经的第十章是什么",
    ];
 

    // 1.set 创建 JoinSet  用于管理多个异步任务的执行
    let mut set = JoinSet::new();

    for prompt in prompts {
        let span = tracing::info_span!("chat", prompt = prompt);
        // 2. 启动异步任务，将结果加入到 JoinSet 中
        set.spawn(async move{
            tracing::info!("\n\nprompt: {}", prompt);
            // 2.1 从信号量中获取许可
            let permit = get_semaphore().acquire().await?;
            // let output = chat_complete(
            //     DEEPSEEK_FLASH_MODEL,
            //     Some("你是一个全能的助手"),
            //     Some(prompt),
            // ).await?;
            let output = chat_stream_with_rety(DEEPSEEK_FLASH_MODEL, Some("你是一个全能的助手"), Some(prompt)).await?;
            // 2.2 释放许可
            drop(permit);

            // 2.3 异步返回的结果，包含 prompt 和 output 两个字段
            Ok::<_, anyhow::Error>((prompt, output))
        }
        .instrument(span)); // instrument 用于添加 tracing 日志
    }

    // 这里result 由2个Result 包裹，第一个是await 等待任务完成，第二个是任务返回的结果
    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok((prompt, output))) => {
                tracing::info!("prompt: {}, output: {}", prompt, output);
            },
            Ok(Err(e)) => {
                tracing::error!("Error: {:?}", e);
            },
            Err(e) => {
                tracing::error!("Error: {:?}", e);
            },
        }
    }



    Ok(())
}


