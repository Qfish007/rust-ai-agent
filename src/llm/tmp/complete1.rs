use async_openai::types::chat::{
    ChatCompletionMessageToolCalls,
    ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestToolMessageArgs,
    ChatCompletionRequestUserMessageArgs,
    ChatCompletionTools,
    CreateChatCompletionRequestArgs,
};

use crate::tools::{ CalculatorArgs, calculator };

pub async fn chat_complete(
    model: &str,
    system: Option<&str>,
    prompt: Option<&str>,
    tools: Vec<ChatCompletionTools>
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
            ChatCompletionRequestSystemMessageArgs::default().content(system).build()?.into()
        );
    }

    // 将用户提示添加到消息队列中
    if let Some(prompt) = prompt {
        messages.push(
            ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?.into()
        );
    }

    // 构建请求
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages.clone())
        .tools(tools.clone())
        .max_tokens(2048u32)
        .build()?;

    // 发送请求
    let response = client.chat().create(request).await?;

    // 打印响应
    tracing::info!("response: {:#?}", response);

    // 拿到消息
    let message = response.choices
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No message found in response"))?.message;

    // 检查是否有工具调用
    if let Some(tool_calls) = message.tool_calls {
        // 将需要调用的工具调用添加到消息队列中
        messages.push(
            ChatCompletionRequestAssistantMessageArgs::default()
                .tool_calls(tool_calls.clone())
                .build()?
                .into()
        );

        // 处理工具调用
        for tool_call in tool_calls {
            // 匹配工具调用类型
            match tool_call {
                ChatCompletionMessageToolCalls::Function(function_call) => {
                    let function_name = function_call.function.name;
                    let arguments = function_call.function.arguments;
                    tracing::info!("function_name: {}, arguments: {}", function_name, arguments);

                    if function_name == "calculator" {
                        let args: CalculatorArgs = serde_json::from_str(&arguments)?;
                        let result = calculator(
                            &args.operator,
                            args.first_number,
                            args.second_number
                        );
                        tracing::info!("calculator result: {:#?}", result);
                        // tracing::info!("calculator args: {:#?}", args);
                        let tool_result = match result {
                            Ok(result) => result.to_string(),
                            Err(error) => error.to_string(),
                        };
                        tracing::info!("calculator tool_result: {:#?}", tool_result);

                        // 将工具调用结果添加到消息队列中
                        messages.push(
                            ChatCompletionRequestToolMessageArgs::default()
                                .tool_call_id(function_call.id.clone())
                                .content(tool_result)
                                .build()?
                                .into()
                        );
                    }
                }
                _ => {
                    tracing::info!("unsupport tool_call: {:#?}", tool_call);
                }
            }
        }

        // 将信息在发送给模型
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(messages.clone())
            .tools(tools.clone())
            .max_tokens(2048u32)
            .build()?;
        let response = client.chat().create(request).await?;
        // tracing::info!("response: {:#?}", response);
        let content = response.choices
            .into_iter()
            .next()
            .and_then(|choice| choice.message.content)
            .ok_or_else(|| anyhow::anyhow!("No content found in response"))?;

        return Ok(content);
    }

    // 从响应中提取内容
    let content = message
    .content
    .ok_or_else(|| anyhow::anyhow!("No content found in response"))?;


    Ok(content)
}
