use schemars::schema_for;
use serde_json::Value;

use crate::{agent::ExecutionContext, tools::{CalculatorArgs, Tool, calculator}};

pub struct CalculatorTool;

#[async_trait::async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Perform basic arithmetic operations."
    }

    fn parameters(&self) -> Value {
        serde_json::to_value(schema_for!(CalculatorArgs))
            .expect("Failed to serialize CalculatorArgs::name")
    }

    async fn execute(&self, args_json: &str, _context: &ExecutionContext) -> anyhow::Result<String> {
        let args: CalculatorArgs = serde_json::from_str(args_json)?;
        let result = calculator(&args.operator, args.first_number, args.second_number);
        match result {
            Ok(result) => Ok(format!(
                "{} {} = {}",
                args.operator, args.first_number, result
            )),
            Err(err) => Ok(format!("Error: {}", err)),
        }
    }
}
