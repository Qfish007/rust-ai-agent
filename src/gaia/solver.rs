use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
    FinishReason,
    ResponseFormat
};
use backon::{ ExponentialBuilder, Retryable };

use crate::gaia::GaiaOutput;

pub const GAIA_PROMPT: &'static str =
    r#"You are a general AI assistant. I will ask you a question.
First, determine if you can solve this problem with your current capabilities and set "is_solvable" accordingly.
If you can solve it, set "is_solvable" to true and provide your answer in "final_answer".
If you cannot solve it, set "is_solvable" to false and explain why in "unsolvable_reason".
Your final answer should be a number OR as few words as possible OR a comma-separated list of numbers and/or strings.
If you are asked for a number, don't use a comma to write your number neither use units such as $ or percent sign unless sp
If you are asked for a string, don't use articles, neither abbreviations (e.g., for cities), and write the digits in plain
If you are asked for a comma-separated list, apply the above rules depending on whether the element is a number or a string
"#;

// solve_problem_with_retry, 重试3次
#[allow(unused)]
pub async fn solve_problem_with_retry(
    model: &str,
    system: &str,
    prompt: &str
) -> anyhow::Result<GaiaOutput> {
    let op = || async { solve_problem(model, system, prompt).await };
    op.retry(ExponentialBuilder::default().with_max_times(3)).await
}

async fn solve_problem(model: &str, system: &str, prompt: &str) -> anyhow::Result<GaiaOutput> {
    let schema = schemars::schema_for!(GaiaOutput);
    let schema_json = serde_json::to_string_pretty(schema.as_value())?;
    let schema_str = format!("你必须以 JSON 格式回复，输出需严格符合以下 JSON Schema，不要包含任何其他内容：\n{schema_json}");

    // 2. 构建请求
    let client = async_openai::Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default().content(system).build()?.into(),
            ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?.into(),
            ChatCompletionRequestSystemMessageArgs::default().content(schema_str).build()?.into()
        ]).response_format(ResponseFormat::JsonObject)
        .build()?;

    // 3. 发送请求
    let response = client.chat().create(request).await?;
    let choice = response.choices
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No choice in response"))?;

    // 4. 解析响应,检查模型是否拒绝回答
    if choice.finish_reason == Some(FinishReason::ContentFilter) {
        return Ok(GaiaOutput {
            is_solvable: false,
            unsolvable_reason: "Model refuse to answer".to_string(),
            final_answer: String::new(),
        });
    }

    // 5. 解析响应,将模型回答转换为GaiaOutput结构体
    let content = choice.message.content.ok_or_else(|| anyhow::anyhow!("No content in message"))?;
    let output: GaiaOutput = serde_json::from_str(&content)?;

    Ok(output)
}
