use std::{ collections::HashMap, sync::Arc };

use ai_agent::{
    constant::DEEPSEEK_FLASH_MODEL,
    gaia::*,
    llm::*,
    tools::build_toolbox,
};
use tokio::task::JoinSet;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    gaia_level1_experiment().await
}

pub async fn gaia_level1_experiment() -> anyhow::Result<()> {
    // 加载GAIA Level1数据集
    let problems = load_gaia_level1().await?;
    // 构建工具箱
    let toolbox = Arc::new(build_toolbox().await?);

    // 启动任务集
    let mut set = JoinSet::new();

    // 无工具评估
    for problem in problems.iter() {
        let problem = problem.clone();
        set.spawn(async move {
            let permit = get_semaphore().acquire().await?;
            let eval = evaluate_gaia_single(problem, DEEPSEEK_FLASH_MODEL).await;
            drop(permit);
            Ok::<_, anyhow::Error>(("without_tools", eval))
        });
    }

    // 带工具评估
    for problem in problems.iter() {
        let problem = problem.clone();
        let toolbox = toolbox.clone();
        set.spawn(async move {
            let permit = get_semaphore().acquire().await?;
            let eval = evaluate_gaia_single_with_tools(problem, DEEPSEEK_FLASH_MODEL, toolbox).await;
            drop(permit);
            Ok::<_, anyhow::Error>(("with_tools", eval))
        });
    }

    let mut results: HashMap<&str, Vec<GaiaEvalResult>> = HashMap::new();
    while let Some(Ok(result)) = set.join_next().await {
        match result {
            Ok((group, eval)) => {
                tracing::info!("[{group}] {eval:#?}");
                results.entry(group).or_default().push(eval);
            }
            Err(e) => tracing::error!("task panicked: {e}"),
        }
    }

    tracing::info!("=== 带工具 vs 不带工具 ===");
    for group in ["with_tools", "without_tools"] {
        if let Some(evals) = results.get(group) {
            let correct = evals
                .iter()
                .filter(|e| e.correct)
                .count();
            let total = evals.len();
            tracing::info!(
                "{group}: {correct}/{total} ({:.1}%)",
                ((correct as f64) / (total as f64)) * 100.0
            );
        }
    }

    Ok(())
}
