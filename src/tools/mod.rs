use std::{collections::HashMap, sync::Arc};

mod calculator;
mod tool_trait;
mod web_search;
mod mcp;

mod file_unzip;
mod file_read;
mod file_list;
mod file_delete;
mod read_image;

pub use calculator::*;
pub use tool_trait::Tool;
pub use web_search::*;
pub use mcp::*;

pub use file_unzip::*;
pub use file_read::*;
pub use file_list::*;
pub use read_image::*;
pub use file_delete::*;



// 工具箱
pub type ToolBox = HashMap<String, Box<dyn Tool>>;

pub async fn build_toolbox() -> anyhow::Result<ToolBox> {
    let mut tools: Vec<Box<dyn Tool + 'static>> = vec![Box::new(CalculatorTool), Box::new(WebSearchTool)];

    let mcp_client = Arc::new(McpClient::connect().await?);
    for tool in mcp_client.list_tools().await? {
        let x = Box::new(McpTool::new(mcp_client.clone(), tool));
        tools.push(x);
    }

    Ok(into_toolbox(tools))
}

pub fn build_file_explorer_toolbox(vision_model: impl Into<String>) -> ToolBox {
    let tools: Vec<Box<dyn Tool>> = vec![
        Box::new(UnzipFileTool),
        Box::new(ListFileTool),
        Box::new(ReadFileTool),
        Box::new(ReadImageTool::new(vision_model)),
        Box::new(DeleteFileTool),
    ];
    into_toolbox(tools)
}

fn into_toolbox(tools: Vec<Box<dyn Tool>>) -> ToolBox {
    tools.into_iter().map(|t| (t.name().to_string(), t)).collect()
}