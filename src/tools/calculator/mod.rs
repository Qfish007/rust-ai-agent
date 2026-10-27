/*
 * 计算器工具
 * 因为所有的工具内部都是命名 definition 和 execute，
 * 这里不要用 pub mod execute, pub mod definition;
 * 直接 mod execute, mod definition; 防止重复导出
 * 
 * 下面 pub use execute::*; 如果内部有相同的名称，也要注意避免冲突
 */
mod execute;
mod definition;
mod r#impl; // impl是关键词，所以用 r#impl 来避免冲突

#[allow(unused_imports,)]
pub use definition::calculator_tool_definition;
#[allow(unused_imports,)]
pub use execute::*;

pub use r#impl::CalculatorTool;


