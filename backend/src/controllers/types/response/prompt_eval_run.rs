use serde::{Deserialize, Serialize};

use crate::db::types::prompt_eval_run::{PromptEvalRun, PromptEvalVersionPerformance};

// GET EVAL RESPONSE
#[derive(Debug, Serialize, Deserialize)]
pub struct PromptEvalRunResponse {
    pub id: i64,
    pub run_id: String,
    pub prompt_version_id: i64,
    pub prompt_eval_id: i64,
    pub prompt_eval_name: String,
    pub score: Option<i64>,
    pub output: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<PromptEvalRun> for PromptEvalRunResponse {
    fn from(run: PromptEvalRun) -> Self {
        PromptEvalRunResponse {
            id: run.id,
            run_id: run.run_id,
            prompt_version_id: run.prompt_version_id,
            prompt_eval_id: run.prompt_eval_id,
            prompt_eval_name: run.prompt_eval_name,
            score: run.score,
            output: run.output,
            created_at: run.created_at.to_string(),
            updated_at: run.updated_at.to_string(),
        }
    }
}

// EXECUTION RESPONSE
#[derive(Debug, Serialize, Deserialize)]
pub struct PromptEvalExecutionRunResponse {
    pub run_id: String,
    pub runs: Vec<PromptEvalRunResponse>
}

impl From<Vec<PromptEvalRun>> for PromptEvalExecutionRunResponse {
    fn from(runs: Vec<PromptEvalRun>) -> Self {
        let run_id = runs.first().expect("Requires at least 1 run").run_id.clone();
        let runs = runs.into_iter().map(|r| {
            r.into()
        }).collect::<Vec<PromptEvalRunResponse>>();

        PromptEvalExecutionRunResponse {
            run_id,
            runs
        }
    }
}


// PERFORMANCE
#[derive(Debug, Serialize, Deserialize)]
pub struct PromptEvalVersionPerformanceResponse {
    pub version_id: i64,
    pub version_number: i64,
    pub version_date: String,
    pub avg_score: Option<f64>,
    pub run_count: i64,
}

impl From<PromptEvalVersionPerformance> for PromptEvalVersionPerformanceResponse {
    fn from(res: PromptEvalVersionPerformance) -> Self {
        PromptEvalVersionPerformanceResponse {
            version_id: res.version_id,
            version_number: res.version_number,
            version_date: res.version_date.to_string(),
            avg_score: res.avg_score,
            run_count: res.run_count
        }
    }
}
