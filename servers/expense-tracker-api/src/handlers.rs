use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use chrono::Datelike;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::*;
use crate::store::Db;

fn matches_month(date: &chrono::NaiveDate, month: &Option<String>) -> bool {
    match month {
        Some(m) => format!("{:04}-{:02}", date.year(), date.month()) == *m,
        None => true,
    }
}

fn matches_category(category: &str, filter: &Option<String>) -> bool {
    match filter {
        Some(c) => category.eq_ignore_ascii_case(c),
        None => true,
    }
}

/// GET /expenses?category=Food&month=2026-07
pub async fn list_expenses(
    State(db): State<Db>,
    Query(params): Query<ExpenseQuery>,
) -> Json<Vec<Expense>> {
    let store = db.read().await;

    let mut items: Vec<Expense> = store
        .values()
        .filter(|e| {
            matches_category(&e.category, &params.category) && matches_month(&e.date, &params.month)
        })
        .cloned()
        .collect();

    items.sort_by_key(|e| e.date);
    Json(items)
}

/// GET /expenses/{id}
pub async fn get_expense(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
) -> Result<Json<Expense>, AppError> {
    let store = db.read().await;
    store.get(&id).cloned().map(Json).ok_or(AppError::NotFound)
}

/// POST /expenses
pub async fn create_expense(
    State(db): State<Db>,
    Json(payload): Json<CreateExpense>,
) -> Result<(StatusCode, Json<Expense>), AppError> {
    if payload.amount < 0.0 {
        return Err(AppError::BadRequest("amount must be >= 0".into()));
    }
    if payload.description.trim().is_empty() {
        return Err(AppError::BadRequest("description must not be empty".into()));
    }

    let id = Uuid::new_v4();
    let expense = Expense {
        id,
        description: payload.description,
        amount: payload.amount,
        category: payload.category,
        date: payload.date,
    };

    db.write().await.insert(id, expense.clone());
    Ok((StatusCode::CREATED, Json(expense)))
}

/// PUT /expenses/{id} — partial update, only supplied fields change.
pub async fn update_expense(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateExpense>,
) -> Result<Json<Expense>, AppError> {
    let mut store = db.write().await;
    let expense = store.get_mut(&id).ok_or(AppError::NotFound)?;

    if let Some(description) = payload.description {
        expense.description = description;
    }
    if let Some(amount) = payload.amount {
        if amount < 0.0 {
            return Err(AppError::BadRequest("amount must be >= 0".into()));
        }
        expense.amount = amount;
    }
    if let Some(category) = payload.category {
        expense.category = category;
    }
    if let Some(date) = payload.date {
        expense.date = date;
    }

    Ok(Json(expense.clone()))
}

/// DELETE /expenses/{id}
pub async fn delete_expense(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let mut store = db.write().await;
    store.remove(&id).ok_or(AppError::NotFound)?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /expenses/summary?month=2026-07 — total + per-category breakdown.
/// This is the one handler that goes beyond plain CRUD: it's the
/// aggregation tool an MCP client would call to answer "how much did
/// I spend on X last month" in one round trip instead of pulling every
/// row and summing client-side.
pub async fn summary(
    State(db): State<Db>,
    Query(params): Query<ExpenseQuery>,
) -> Json<SummaryResponse> {
    let store = db.read().await;

    let filtered: Vec<&Expense> = store
        .values()
        .filter(|e| matches_month(&e.date, &params.month))
        .collect();

    let mut totals: HashMap<String, (f64, usize)> = HashMap::new();
    for e in &filtered {
        let entry = totals.entry(e.category.clone()).or_insert((0.0, 0));
        entry.0 += e.amount;
        entry.1 += 1;
    }

    let total: f64 = filtered.iter().map(|e| e.amount).sum();

    let mut by_category: Vec<CategorySummary> = totals
        .into_iter()
        .map(|(category, (total, count))| CategorySummary { category, total, count })
        .collect();
    by_category.sort_by(|a, b| b.total.partial_cmp(&a.total).unwrap());

    Json(SummaryResponse {
        month: params.month,
        total,
        by_category,
    })
}
