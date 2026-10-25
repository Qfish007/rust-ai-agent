use std::collections::HashMap;

use async_openai::types::chat::ChatCompletionTools;

pub mod calculator;
pub mod web_search;
pub mod tools_trait;

pub use calculator::{*, CalculatorTool};
pub use web_search::{*, WebSearchTool};


pub use tools_trait::ToolsTrait;


// 定义工具
pub fn tools() -> Vec<ChatCompletionTools> {
    return vec![calculator_tool_definition(), web_search_tool_definition()];
}

pub type ToolBox = HashMap<String, Box<dyn ToolsTrait>>;

pub fn build_toolbox() -> ToolBox {
    let tools:Vec<Box<dyn ToolsTrait>> = vec![Box::new(CalculatorTool), Box::new(WebSearchTool)];
    
    tools.into_iter().map(|tool| (tool.name().to_string(), tool)).collect()
        
}

