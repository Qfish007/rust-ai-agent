use std::collections::HashMap;

mod calculator;
mod tool_trait;
mod web_search;

pub use calculator::*;
pub use tool_trait::Tool;
pub use web_search::*;

// 工具箱
pub type ToolBox = HashMap<String, Box<dyn Tool>>;

// 构建工具箱
pub async fn build_toolbox() -> anyhow::Result<ToolBox> {
    let tools: Vec<Box<dyn Tool>> = vec![Box::new(CalculatorTool), Box::new(WebSearchTool)];

    Ok(tools
        .into_iter()
        .map(|tool| (tool.name().to_string(), tool))
        .collect())
}
