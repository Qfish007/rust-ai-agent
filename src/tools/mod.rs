pub mod calculator;

use async_openai::types::chat::ChatCompletionTools;
#[allow(unused_imports)]
pub use calculator::*;

// 定义工具
pub fn tools() -> Vec<ChatCompletionTools> {
    return vec![calculator_tool_definition()];
}
