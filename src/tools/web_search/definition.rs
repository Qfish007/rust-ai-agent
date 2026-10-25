use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools,FunctionObjectArgs};
use serde_json::json;

pub fn web_search_tool_definition()->ChatCompletionTools{
    ChatCompletionTools::Function(
        ChatCompletionTool{
            function:FunctionObjectArgs::default()
            .name("web_search")
            .description("Search the web for information.")
            .parameters(json!({ 
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The query to search the web for."
                    },
                    "max_results":{
                        "type":"number",
                        "description":"The maximum number of results to return.",
                        "default": 5,
                        "minimum": 0,
                        "maximum": 20,
                    },
                    "topic":{
                        "type":"string",
                        "description":"use news for recent events, sport ,politics, entertainment, technology, health, science, and more.",
                        "enum":["general","news","finance"],
                        "default":"general",
                    },
                    "time_range":{
                        "type":"string",
                        "description":"The time range to search for results.",
                        "enum":["all","day","week","month","year"],
                        "default":"all",
                    },
                },
                "required": ["query"]
            }))
            .build()
            .expect("failed to build web_search tool definition")
        })
}