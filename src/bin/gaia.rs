use std::collections::HashMap;

use ai_agent::{
    gaia::{GaiaEvalResult, evaluate_gaia_single, load_gaia_level1},
    llm::get_semaphore,
};
use tokio::task::JoinSet;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

const MODELS: &[&str] = &["deepseek-v4-flash", "deepseek-v4-pro"];

pub async fn gaia_level1_experiment() -> anyhow::Result<()> {
    // step1: 加载数据集
    let problems = load_gaia_level1().await?;
    // step2: JoinSet 用于并行评估多个模型
    let mut set = JoinSet::new();

    
    for model in MODELS.iter() {
        for problem in problems.iter() {
            let problem = problem.clone();
            // step3: 并行评估多个模型
            set.spawn(async move {
                // step4: 设置信号量
                let permit = get_semaphore().acquire().await?;
                // step5: 评估模型
                let eval = evaluate_gaia_single(problem, model).await;
                drop(permit);
                Ok::<_, anyhow::Error>(eval)
            });
        }
    }

    let mut results: HashMap<String, Vec<GaiaEvalResult>> = HashMap::new();
    while let Some(Ok(result)) = set.join_next().await {
        match result {
            Ok(eval) => {
                tracing::info!("{:#?}", eval);
                results.entry(eval.model.clone()).or_default().push(eval);
            }
            Err(e) => {
                tracing::error!("Error: {:?}", e);
            }
        }
    }

    for (model, evals) in &results {
        let correct = evals.iter().filter(|e| e.correct).count();
        let total = evals.len();
        tracing::info!(
            "{}: {} / {} ({:.2}%)",
            model,
            correct,
            total,
            correct as f64 / total as f64
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    gaia_level1_experiment().await
}
