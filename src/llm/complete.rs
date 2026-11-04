use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionTools, CreateChatCompletionRequestArgs,
};

use crate::tools::{CalculatorArgs, calculator};

pub async fn chat_complete(
    model: &str,
    system: Option<&str>,
    prompt: Option<&str>,
    tools: Vec<ChatCompletionTools>,
) -> anyhow::Result<String> {
    // 创建 AI 客户端
    let client = async_openai::Client::new();

    let mut messages = vec![];

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
        .messages(messages.clone())
        .tools(tools.clone())
        .max_tokens(2048u32)
        .build()?;

    // 发送请求
    let response = client.chat().create(request).await?;
    tracing::info!("response: {:#?}", response);

    let message = response
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No message found in response"))?
        .message;

    if let Some(tool_calls) = message.tool_calls {

        messages.push(
            ChatCompletionRequestAssistantMessageArgs::default()
                .tool_calls(tool_calls.clone())
                .build()?
                .into(),
        );

        for tool_call in tool_calls {
            match tool_call {
                ChatCompletionMessageToolCalls::Function(function_call) => {
                    let fun_name = function_call.function.name;
                    let fun_args = function_call.function.arguments;
                    tracing::info!("fun_name: {}, fun_args: {}", fun_name, fun_args);

                    if fun_name == CalculatorArgs::name() {
                        let args: CalculatorArgs = serde_json::from_str(&fun_args)?;
                        let result = calculator(&args.operator, args.first_number, args.second_number);

                        let tool_result = match result {
                            Ok(result) => result.to_string(),
                            Err(error) => error.to_string(),
                        };

                        messages.push(
                            ChatCompletionRequestToolMessageArgs::default()
                                .tool_call_id(function_call.id.clone())
                                .content(tool_result)
                                .build()?
                                .into(),
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

        let content = response
            .choices
            .into_iter()
            .next()
            .and_then(|choice| choice.message.content)
            .ok_or_else(|| anyhow::anyhow!("No content found in response"))?;

        return Ok(content);
    }

    // 没有工具调用，直接返回消息内容
    let content = message
        .content
        .ok_or_else(|| anyhow::anyhow!("No content found in response"))?;

    Ok(content)
}
