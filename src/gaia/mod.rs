pub mod models;
pub mod dataset;
pub mod solver;
pub mod evaluator;



#[allow(unused_imports)]
pub use models::*;
#[allow(unused_imports)]
pub use dataset::*;
#[allow(unused_imports)]
pub use solver::*;
#[allow(unused_imports)]
pub use evaluator::*;

// model 是模型的名称
// dataset 是数据集的名称，里面有问题和答案
// solver 通过模型求解问题
// evaluator 评估模型的性能
