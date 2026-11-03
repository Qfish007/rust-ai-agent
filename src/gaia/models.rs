use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// huggingface 响应结构体
#[derive(Deserialize, Debug,Serialize,JsonSchema)]
pub struct HfResponse {
    pub rows:Vec<HfRow>,
}

// huggingface 行结构体
#[derive(Deserialize, Debug,Serialize,JsonSchema)]
pub struct HfRow {
    pub row:GaiaRow,
}

// gaia 行结构体
#[derive(Deserialize, Debug,Clone,Serialize,JsonSchema)]
pub struct GaiaRow {
    pub task_id:String,
    #[serde(rename = "Question")]
    pub question:String,
    #[serde(rename = "Level")]
    pub level:String,
    #[serde(rename = "Final answer")]
    pub final_answer:String,
}

// gaia 大模型输出结构体
#[derive(Deserialize, Debug,Serialize,JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GaiaOutput {
    pub is_solvable:bool,
    pub unsolvable_reason:String,
    pub final_answer:String,
}

// gaia 评估结果结构体
#[derive(Deserialize, Debug)]
pub struct GaiaEvalResult {
    pub task_id:String,
    pub model:String,
    pub correct:bool,
    pub is_solvable:Option<bool>,
    pub prediction:Option<String>,
    pub answer:String,
    pub unsolvable_reason:Option<String>,
    pub error:Option<String>,
}