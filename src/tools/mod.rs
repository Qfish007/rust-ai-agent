pub mod calculator;
pub mod web_search;



use async_openai::types::chat::ChatCompletionTools;

pub use calculator::*;
pub use web_search::*;

// 定义工具
pub fn tools() -> Vec<ChatCompletionTools> {
    return vec![calculator_tool_definition(), web_search_tool_definition()];
}
