use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_stream::stream;
use futures::{ Stream, StreamExt };
use backon::{ ExponentialBuilder, Retryable };

pub fn chat_stream(
    model: &str,
    system: Option<&str>,
    prompt: Option<&str>
) -> impl Stream<Item = anyhow::Result<String>> {
    stream! {
        // 1. 创建 AI 客户端
        let client = async_openai::Client::new();

        let mut messages = vec![];

        // 2. 添加系统信息到消息队列中
        if let Some(system) = system {
            messages.push(
                ChatCompletionRequestSystemMessageArgs::default().content(system).build()?.into()
            );
        }

        // 3. 添加用户提示到消息队列中
        if let Some(prompt) = prompt {
            messages.push(
                ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?.into()
            );
        }

        // 4. 构建请求
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(messages)
            .max_tokens(4096u32)
            .build()?;

        // 5. 发送请求
        let mut stream = client.chat().create_stream(request).await?;

        // 6. 处理响应
        while let Some(response_result) = stream.next().await {
            match response_result {
                Ok(chunk) => {
                    if
                        let Some(choice) = chunk.choices.first() &&
                        let Some(new_text) = &choice.delta.content
                    {
                        yield Ok(new_text.clone());
                    }
                }
                Err(e) => {
                    yield Err(e.into());
                }
            }
        }
    }
}

pub async fn chat_stream_with_rety(
    model: &str,
    system: Option<&str>,
    prompt: Option<&str>
) -> anyhow::Result<String> {
    let op = || async {
        let s = chat_stream(model, system, prompt);

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

        Ok(output)
    };

    op.retry(ExponentialBuilder::default().with_max_times(3)).await
}
