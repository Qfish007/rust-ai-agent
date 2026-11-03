use async_openai::Client;
use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};

pub async fn chat_complete(
    model: &str,
    system: Option<&str>,
    prompt: Option<&str>,
) -> anyhow::Result<String> {
    // 创建 AI 客户端
    let client = Client::new();

    // 消息队列
    let mut messages = vec![];

    // 添加系统信息
    if let Some(system) = system {
        messages.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system)
                .build()?
                .into(),
        );
    }

    // 添加用户提示
    if let Some(prompt) = prompt {
        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        );
    }

    // 构建请求
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages.clone())
        .max_tokens(2048u32)
        .build()?;

    // 发送请求
    let response = client.chat().create(request).await?;

    tracing::info!("response: {:#?}", response);

    let content = response
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .ok_or_else(|| anyhow::anyhow!("No content found in response"))?;

    Ok(content)
}
