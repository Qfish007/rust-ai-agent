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

use crate::tools::{ CalculatorArgs, WebSearchArgs, calculator, search_web };

pub async fn chat_complete(
    model: &str,
    system: Option<&str>,
    prompt: Option<&str>,
    tools: Vec<ChatCompletionTools>
) -> anyhow::Result<String> {
    // 创建 AI 客户端
    let client  = Client::new();

    // 消息队列
    let mut messages = vec![];

    // 添加系统信息
    add_system_message(&mut messages, system)?;

    // 添加用户提示
    add_user_message(&mut messages, prompt)?;
    
    // 发送请求
    let response = make_request(&client, model, &messages, &tools).await?;
    tracing::info!("response: {:#?}", response);

    // 拿到消息
    let response_message = get_response_message(response)?;

    // 检查是否有工具调用
    let tool_result = check_call_tooling(&response_message, &mut messages, &client, model, &tools).await?;
    if let Some(content) = tool_result {
        return Ok(content);
    }

    // 从响应中提取内容
    let content = response_message.content.ok_or_else(|| anyhow::anyhow!("No content found in response"))?;

    Ok(content)
}


// 添加系统信息到消息队列中
fn add_system_message(messages: &mut Vec<ChatCompletionRequestMessage>, system: Option<&str>) -> anyhow::Result<String>{
    if let Some(system) = system {
        messages.push(
            ChatCompletionRequestSystemMessageArgs::default().content(system).build()?.into()
        );
    }
    Ok(system.unwrap().to_string())
}

// 添加用户提示到消息队列中
fn add_user_message(messages: &mut Vec<ChatCompletionRequestMessage>, prompt: Option<&str>) -> anyhow::Result<String>{
    if let Some(prompt) = prompt {
        messages.push(
            ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?.into()
        );
    }
    Ok(prompt.unwrap().to_string())
}

// 添加工具调用到消息队列中
fn add_tool_message(messages: &mut Vec<ChatCompletionRequestMessage>, tool_calls: &Vec<ChatCompletionMessageToolCalls>) -> anyhow::Result<()>{
    
        messages.push(
            ChatCompletionRequestAssistantMessageArgs::default()
                .tool_calls(tool_calls.clone())
                .build()?
                .into()
        );
        Ok(())
}
// 添加工具调用结果到消息队列中
fn add_tool_result_message(messages: &mut Vec<ChatCompletionRequestMessage>, tool_call_id: &str, tool_result: &str) -> anyhow::Result<()>{
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
async fn make_request(client: &Client<OpenAIConfig>, model: &str, 
    messages: &Vec<ChatCompletionRequestMessage>, 
    tools: &Vec<ChatCompletionTools>) -> anyhow::Result<CreateChatCompletionResponse>{
    
   
        // 构建请求
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages.clone())
        .tools(tools.clone())
        .max_tokens(2048u32)
        .build()?;

    // 发送请求
    let response  = client.chat().create(request).await?;
    Ok(response)
}

// 从响应中提取消息
fn get_response_message(response: CreateChatCompletionResponse) -> anyhow::Result<ChatCompletionResponseMessage>{
    let message = response.choices
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No message found in response"))?
        .message;
    Ok(message)
}

// 将工具的调用独立出来
 async fn check_call_tooling(
    response_message: &ChatCompletionResponseMessage, 
    messages: &mut Vec<ChatCompletionRequestMessage>,
    client: &Client<OpenAIConfig>,
    model: &str,
    tools: &Vec<ChatCompletionTools>,
   ) -> anyhow::Result<Option<String>>{

       if let Some(tool_calls) = response_message.tool_calls.clone() {
        // 将需要调用的工具调用添加到消息队列中
        add_tool_message(messages, &tool_calls)?;

        // 处理工具调用
        for tool_call in tool_calls {
            // 匹配工具调用类型
            match tool_call {
                ChatCompletionMessageToolCalls::Function(function_call) => {
                    let function_name = function_call.function.name;
                    let arguments = function_call.function.arguments;
                    tracing::info!("function_name: {}, arguments: {}", function_name, arguments);

                    if function_name == CalculatorArgs::name() {
                        let args: CalculatorArgs = serde_json::from_str(&arguments)?;
                        let result = calculator(
                            &args.operator,
                            args.first_number,
                            args.second_number
                        );
                        tracing::info!("计算结果: {:#?}", result);
                        let tool_result = match result {
                            Ok(result) => result.to_string(),
                            Err(error) => error.to_string(),
                        };
                        
                        // 将工具调用结果添加到消息队列中
                        add_tool_result_message(messages, &function_call.id, &tool_result)?;
                    } else if function_name == WebSearchArgs::name() {
                        let args: WebSearchArgs = serde_json::from_str(&arguments)?;
                        let result = search_web(args).await;
                        // tracing::info!("搜索结果: {:#?}", result);
                        let tool_result = match result{
                            Ok(result) => serde_json::to_string(&result)?,
                            Err(error) => error.to_string(),
                        };

                        tracing::info!("搜索结果: {}", tool_result);
                        // 将工具调用结果添加到消息队列中
                        add_tool_result_message(messages, &function_call.id, &tool_result)?;
                    }
                }
                _ => {
                    tracing::info!("unsupport tool_call: {:#?}", tool_call);
                }
            }
        }

        let response = make_request(&client, model, &messages, &tools).await?;
        let content = get_response_message(response)?.content.ok_or_else(|| anyhow::anyhow!("No content found in response"))?;

        return Ok(Some(content));
        
    }

    
    Ok(None)
}