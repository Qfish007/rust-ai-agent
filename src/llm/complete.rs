use async_openai::types::chat::{
    ChatCompletionMessageToolCalls,
    ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestToolMessageArgs,
    ChatCompletionRequestUserMessageArgs,
    ChatCompletionTools,
    CreateChatCompletionRequestArgs,
};

use crate::tools::*;

pub async fn chat_complete(
    model: &str,
    system: Option<&str>,
    prompt: Option<&str>,
    tool_box: &ToolBox
) -> anyhow::Result<String> {
    // 创建 AI 客户端
    let client = async_openai::Client::new();

    let mut messages = vec![];

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

    loop {
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
        tracing::info!("🌹llm response: {:#?}🌹", response);

        let message = response.choices
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No message found in response"))?.message;

        // 4 处理工具调用结果
        if let Some(tool_calls) = message.tool_calls {
            // 4.1 添加工具调用到消息队列中
            messages.push(
                ChatCompletionRequestAssistantMessageArgs::default()
                    .tool_calls(tool_calls.clone())
                    .build()? 
                    .into()
            );

            // 4.2 遍历工具调用结果，执行工具调用
            for tool_call in tool_calls {
                if let ChatCompletionMessageToolCalls::Function(function_call) = tool_call {
                    let fun_name = &function_call.function.name;
                    let fun_args = &function_call.function.arguments;
                    tracing::info!("fun_name: {}, fun_args: {}", fun_name, fun_args);

                    let tool_result = match tool_box.get(fun_name) {
                        Some(tool) => { 
                            match tool.execute(fun_args).await {
                                Ok(result) => {
                                    tracing::info!("tool 结果: {:#?}", result);
                                    result
                                }
                                Err(error) => {
                                    let msg = format!("tool execution error: {}", error);
                                    // tracing::error!("Error in tool execution: {:?}", msg);
                                    msg
                                }
                            }
                        }
                        None => {
                            let msg = format!("tool not found: {}", fun_name);
                            // tracing::error!("Error in tool execution: {:?}", msg);
                            msg
                        }
                    };

                    messages.push(
                        ChatCompletionRequestToolMessageArgs::default()
                            .tool_call_id(function_call.id)
                            .content(tool_result)
                            .build()?
                            .into()
                    );
                }
            }
        }else{
            let content = message
            .content
            .ok_or_else(|| anyhow::anyhow!("No content found in response"))?;

            return Ok(content);
        }
    }
}
