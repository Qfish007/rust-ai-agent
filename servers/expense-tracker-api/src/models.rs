use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub description: String,
    pub amount: f64,
    pub category: String,
    pub date: NaiveDate,
}

/// Body for `POST /expenses`.
#[derive(Debug, Deserialize)]
pub struct CreateExpense {
    pub description: String,
    pub amount: f64,
    pub category: String,
    pub date: NaiveDate,
}

/// Body for `PUT /expenses/{id}` — every field is optional so callers
/// can send only what they want to change.
#[derive(Debug, Deserialize)]
pub struct UpdateExpense {
    pub description: Option<String>,
    pub amount: Option<f64>,
    pub category: Option<String>,
    pub date: Option<NaiveDate>,
}

/// Query params shared by `GET /expenses` and `GET /expenses/summary`.
/// `month` is expected as "YYYY-MM".
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct ExpenseQuery {
    pub category: Option<String>,
    pub month: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CategorySummary {
    pub category: String,
    pub total: f64,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct SummaryResponse {
    pub month: Option<String>,
    pub total: f64,
    pub by_category: Vec<CategorySummary>,
}
