use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
};

use crate::models::ActionPlan;

pub async fn chat_complete_structured(
    model: &str,
    system: Option<&str>,
    prompt: Option<&str>,
) -> anyhow::Result<ActionPlan> {
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

    // 下面这段 deepseek 不支持 json_schema 类型的 response_format，只支持 json_object（JSON 模式）
    // JSON 模式不强制遵循 schema，需要在提示词中说明期望的输出格式
    let schema = schemars::schema_for!(ActionPlan);
    let schema_json = schema.as_value().clone();
    let format_settings = ResponseFormat::JsonSchema { 
        json_schema: ResponseFormatJsonSchema{
            description: Some("A step-by-step plan agent action plan with diffifulty and time".to_string()),
            name: "action_plan".to_string(),
            schema: schema_json,
            strict: Some(true),
        }
    };



    // 构建请求
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .response_format(format_settings)
        .max_tokens(2048u32)
        .build()?;

    // 发送请求
    let response = client.chat().create(request).await?;

    // 打印响应
    tracing::info!("response: {:#?}", response);

    // 从响应中提取内容
    let plan: ActionPlan = response
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .ok_or_else(||anyhow::anyhow!("No content found in response"))
        .and_then(|s| serde_json::from_str(&s).map_err(Into::into))?;

    Ok(plan)
}
