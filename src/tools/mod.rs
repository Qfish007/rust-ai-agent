mod calculator;
mod web_search;

pub use calculator::*;
pub use web_search::*;

use async_openai::types::chat::ChatCompletionTools;
// 定义工具列表
pub fn tools() -> Vec<ChatCompletionTools> {
    return vec![calculator_tool_definition(), web_search_tool_definition()];
}
