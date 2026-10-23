

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// 定义想要返回的结构体
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ActionPlan {
    pub plan: String,
    pub steps: Vec<ActionStep>,
    pub difficulty:Difficulty,
    pub estimated_minutes: u32,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ActionStep {
    pub index: u32,
    pub description: String,
    pub tool_hint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}