use std::{collections::HashMap, sync::Arc};

mod calculator;
mod tool_trait;
mod web_search;
mod mcp;

pub use calculator::*;
pub use tool_trait::Tool;
pub use web_search::*;
pub use mcp::*;



// 工具箱
pub type ToolBox = HashMap<String, Box<dyn Tool>>;

// 构建工具箱
pub async fn build_toolbox() -> anyhow::Result<ToolBox> {
    let mut tools: Vec<Box<dyn Tool>> = vec![Box::new(CalculatorTool), Box::new(WebSearchTool)];

    // 创建 MCP 客户端，链接 MCP Server
    let mcp_client = Arc::new(McpClient::connect().await?);

    
    for tool in mcp_client.list_tools().await? {
        // 拿到 MCP Server 暴露的所有工具, 并包装成 McpTool
        tools.push(Box::new(McpTool::new(mcp_client.clone(), tool)));
    }

    Ok(
        tools
            .into_iter()
            .map(|tool| (tool.name().to_string(), tool))
            .collect()
    )
}
