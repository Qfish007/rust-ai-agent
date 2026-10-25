use schemars::schema_for;
use serde_json::Value;

use crate::tools::{WebSearchArgs, ToolsTrait, search_web};

pub struct WebSearchTool;

#[async_trait::async_trait]
impl ToolsTrait for WebSearchTool {
    fn name(&self) -> &str {
        "web_search"
    }

    fn description(&self) -> &str {
        "Search the web for information."
    }

    fn parameters(&self) -> Value {
        serde_json::to_value(schema_for!(WebSearchArgs))
            .expect("Failed to serialize WebSearchArgs schema")
    } 

    async fn execute(&self, args_json: &str) -> anyhow::Result<String> {
       let args:WebSearchArgs = serde_json::from_str(args_json)?;

       match  search_web(args).await{
           Ok(result) => Ok(serde_json::to_string(&result)?),
           Err(err) => Ok(format!("Error: {}", err)),
       }
    }
}
