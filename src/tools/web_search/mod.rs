
mod execute;
mod definition;
mod r#impl;

#[allow(unused_imports,)]
pub use definition::web_search_tool_definition;
#[allow(unused_imports,)]
pub use execute::*;
pub use r#impl::WebSearchTool;
