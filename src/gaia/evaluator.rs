use crate::gaia::{GAIA_PROMPT, GaiaRow, GaiaEvalResult, solve_problem_with_retry};

// 判断预测是否正确
fn is_correct(prediction: &str, answer: &str) -> bool {
    if prediction.is_empty() {
        false
    }else {
        prediction.trim().to_lowercase() == answer.trim().to_lowercase()
    }
}

// 评估GAIA-2023 Level1问题 
// problem: 问题行
// model: 模型名称
pub async fn evaluate_gaia_single(problem:GaiaRow,model:&str) -> GaiaEvalResult {
    // 请求模型求解问题
    let result = solve_problem_with_retry(model, GAIA_PROMPT, &problem.question).await;

    // 判断模型是否成功求解问题
    match result{
        Ok(output) => {
            GaiaEvalResult {
                task_id: problem.task_id,
                model: model.to_string(),
                correct: is_correct(&output.final_answer, &problem.final_answer),
                is_solvable: Some(output.is_solvable),
                prediction: Some(output.final_answer),
                answer: problem.final_answer,
                unsolvable_reason: Some(output.unsolvable_reason),
                error: None,
            }
        }
        Err(err) => {
            GaiaEvalResult {
                task_id: problem.task_id,
                model: model.to_string(),
                correct: false,
                is_solvable: None,
                prediction: None,
                answer: problem.final_answer,
                unsolvable_reason: None,
                error: Some(err.to_string()),
            }
        }
    }
}