use async_openai::types::chat::{
    ChatCompletionMessageToolCalls,
    ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestToolMessageArgs,
    ChatCompletionRequestUserMessageArgs,
    ChatCompletionResponseMessage,
    ChatCompletionTools,
    CreateChatCompletionRequestArgs,
    CreateChatCompletionResponse,
};
use async_openai::config::OpenAIConfig;
use async_openai::Client;

use crate::tools::{ ToolBox };

pub async fn chat_complete(
    model: &str,
    system: Option<&str>,
    prompt: Option<&str>,
    tool_box: &ToolBox
) -> anyhow::Result<String> {
    // 创建 AI 客户端
    let client = Client::new();

    // 消息队列
    let mut messages = vec![];

    // 添加系统信息
    add_system_message(&mut messages, system)?;

    // 添加用户提示
    add_user_message(&mut messages, prompt)?;

    loop {
        // 发送请求
        let response = make_request(&client, model, &messages, &tool_box).await?;
        tracing::info!("response: {:#?}", response);

        // 拿到消息
        let response_message = get_response_message(response)?;

        match check_call_tooling(&response_message, &mut messages, tool_box).await? {
            // 没有工具调用，模型给出了最终答案
            Some(content) => return Ok(content),
            // 本轮执行了工具调用，回到循环重新请求模型
            None => continue,
        }
    }
}

// 添加系统信息到消息队列中
fn add_system_message(
    messages: &mut Vec<ChatCompletionRequestMessage>,
    system: Option<&str>
) -> anyhow::Result<String> {
    if let Some(system) = system {
        messages.push(
            ChatCompletionRequestSystemMessageArgs::default().content(system).build()?.into()
        );
    }
    Ok(system.unwrap().to_string())
}

// 添加用户提示到消息队列中
fn add_user_message(
    messages: &mut Vec<ChatCompletionRequestMessage>,
    prompt: Option<&str>
) -> anyhow::Result<String> {
    if let Some(prompt) = prompt {
        messages.push(
            ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?.into()
        );
    }
    Ok(prompt.unwrap().to_string())
}

// 添加工具调用到消息队列中
fn add_tool_message(
    messages: &mut Vec<ChatCompletionRequestMessage>,
    tool_calls: &Vec<ChatCompletionMessageToolCalls>
) -> anyhow::Result<()> {
    messages.push(
        ChatCompletionRequestAssistantMessageArgs::default()
            .tool_calls(tool_calls.clone())
            .build()?
            .into()
    );
    Ok(())
}
// 添加工具调用结果到消息队列中
fn add_tool_result_message(
    messages: &mut Vec<ChatCompletionRequestMessage>,
    tool_call_id: &str,
    tool_result: &str
) -> anyhow::Result<()> {
    messages.push(
        ChatCompletionRequestToolMessageArgs::default()
            .tool_call_id(tool_call_id)
            .content(tool_result)
            .build()?
            .into()
    );
    Ok(())
}

// 发送请求
async fn make_request(
    client: &Client<OpenAIConfig>,
    model: &str,
    messages: &Vec<ChatCompletionRequestMessage>,
    tool_box: &ToolBox
) -> anyhow::Result<CreateChatCompletionResponse> {
    let tool_definitions: Vec<ChatCompletionTools> = tool_box
        .values()
        .filter_map(|t| {
            match t.definition() {
                Ok(def) => Some(def),
                Err(e) => {
                    tracing::error!("Error in tool definition: {:?}", e);
                    None
                }
            }
        })
        .collect();
    // 构建请求
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages.clone())
        .tools(tool_definitions.clone())
        .max_tokens(2048u32)
        .build()?;

    // 发送请求
    let response = client.chat().create(request).await?;
    Ok(response)
}

// 从响应中提取消息
fn get_response_message(
    response: CreateChatCompletionResponse
) -> anyhow::Result<ChatCompletionResponseMessage> {
    let message = response.choices
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No message found in response"))?.message;
    Ok(message)
}

// 将工具的调用独立出来
// 返回 None 表示本轮有工具调用，消息已追加，调用方应继续下一轮
// 返回 Some(content) 表示没有工具调用，content 是模型的最终回答
async fn check_call_tooling(
    response_message: &ChatCompletionResponseMessage,
    messages: &mut Vec<ChatCompletionRequestMessage>,
    tool_box: &ToolBox,
) -> anyhow::Result<Option<String>> {

    // 检查是否有工具调用
    let Some(tool_calls) = response_message.tool_calls.clone() else {
        let content = response_message
            .content
            .clone()
            .ok_or_else(|| anyhow::anyhow!("No content found in response"))?;
        return Ok(Some(content));
    };

    // 将需要调用的工具调用添加到消息队列中
    add_tool_message(messages, &tool_calls)?;

    // 必须为每个 tool_call 都追加一条结果，否则下一轮请求会被 API 拒绝
    for tool_call in tool_calls {
        if let ChatCompletionMessageToolCalls::Function(function_call) = tool_call {
            let function_name = &function_call.function.name;
            let arguments = &function_call.function.arguments;
            tracing::info!("function_name: {}, arguments: {}", function_name, arguments);

            let tool_result = match tool_box.get(function_name) {
                Some(tool) => {
                    match tool.execute(arguments).await {
                        Ok(result) => {
                            tracing::info!("tool 结果: {:#?}", result);
                            result
                        }
                        Err(error) => {
                            let msg = format!("tool execution error: {}", error);
                            tracing::error!("Error in tool execution: {:?}", msg);
                            msg
                        }
                    }
                }
                None => {
                    let msg = format!("tool not found: {}", function_name);
                    tracing::error!("Error in tool execution: {:?}", msg);
                    msg
                }
            };
            add_tool_result_message(messages, &function_call.id, &tool_result)?;
        }
    }

    Ok(None)
}
