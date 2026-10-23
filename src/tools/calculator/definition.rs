use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools,FunctionObjectArgs};
use serde_json::json;

pub fn calculator_tool_definition()->ChatCompletionTools{
    ChatCompletionTools::Function(
        ChatCompletionTool{
            function:FunctionObjectArgs::default()
            .name("calculator")
            .description("Perform basic arithmetic operations.")
            .parameters(json!({
                "type": "object",
                "properties": {
                    "operator": {
                        "type": "string",
                        "description": "Arithmetic operator to perform.",
                        "enum":["add","subtract","multiply","divide"]
                    },
                    "first_number":{
                        "type":"number",
                        "description":"The first number in the operation."
                    },
                    "second_number":{
                        "type":"number",
                        "description":"The second number in the operation."
                    }
                },
                "required": ["operator","first_number","second_number"]
            }))
            .build()
            .expect("failed to build calculator tool definition")
        })
}