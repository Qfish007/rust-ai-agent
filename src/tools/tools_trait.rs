use anyhow::Ok;
use serde_json::Value;
use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools, FunctionObjectArgs};

#[async_trait::async_trait]
pub trait ToolsTrait:Send + Sync {
    fn name(&self) -> &str;

    fn description(&self) -> &str;
    
    fn parameters(&self) -> Value;

    async fn execute(&self, args_json: &str) -> anyhow::Result<String>;

    // 定义工具
    fn definition(&self) -> anyhow::Result<ChatCompletionTools>{
        let function = FunctionObjectArgs::default()
        .name(self.name())
        .description(self.description())
        .parameters(self.parameters())
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build function: {}, {}", self.name(), e))?;
    
        Ok(ChatCompletionTools::Function(ChatCompletionTool{function}))
    }
}