use std::{collections::HashMap, sync::Arc};

use async_openai::types::chat::ChatCompletionTools;

pub mod calculator;
pub mod web_search;
pub mod tools_trait;
pub mod mcp;

pub use calculator::{*, CalculatorTool};
pub use web_search::{*, WebSearchTool};
pub use mcp::{*};


pub use tools_trait::ToolsTrait;

pub type ToolBox = HashMap<String, Box<dyn ToolsTrait>>;

// 定义工具
pub fn tools() -> Vec<ChatCompletionTools> {
    return vec![calculator_tool_definition(), web_search_tool_definition()];
}



pub async fn build_toolbox() -> anyhow::Result<ToolBox> {
    let mut tools:Vec<Box<dyn ToolsTrait>> = vec![Box::new(CalculatorTool), Box::new(WebSearchTool)];

    // 创建 MCP 客户端，链接 MCP Server
    let mcp_client = Arc::new(McpClient::connect().await?);

    // 拿到 MCP Server 暴露的所有工具
    for tool in mcp_client.list_tools().await? {
        tools.push(Box::new(McpTool::new(mcp_client.clone(), tool)));
    }
    
    Ok(tools
        .into_iter()
        .map(|tool| (tool.name().to_string(), tool)).collect())
        
}

