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
    let client = async_openai::Client::new();

    // 消息队列
    let mut messages = vec![];

    // 角色:有系统信息，用户输入，模型的回复
    // system 身份 能力 行为规范
    // user 用户输入
    // assistant 模型回复
    // 将系统信息添加到消息队列中
    if let Some(system) = system {
        messages.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system)
                .build()?
                .into(),
        );
    }

    // 将用户提示添加到消息队列中
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
        .messages(messages)
        .max_tokens(2048u32)
        .build()?;

    // 发送请求
    let response = client.chat().create(request).await?;

    // 打印响应
    tracing::info!("response: {:#?}", response);

    // 从响应中提取内容
    let content = response
        .choices
        .into_iter()
        .next()
        .and_then(|choice| choice.message.content)
        .unwrap_or_default();

    Ok(content)
}
