use sqlx::{Pool, Postgres};

use super::budget_record::BudgetRecord;

pub(crate) async fn get_all(pool: &Pool<Postgres>) -> Vec<BudgetRecord> {
    sqlx::query_as::<_,BudgetRecord>("SELECT * FROM budget_record")
        .fetch_all(pool).await.expect("something went wrong with the query")
}

pub async fn create(budget_record: &BudgetRecord, pool: &Pool<Postgres>) -> Option<BudgetRecord> {
    Some(sqlx::query_as::<_,BudgetRecord>("INSERT INTO budget_record (date, type, category_id, amount, details) VALUES($1,$2,$3,$4,$5) RETURNING *;")
        .bind(&budget_record.date)
        .bind(i16::from(&budget_record.budget_type))
        .bind(budget_record.category_id)
        .bind(budget_record.amount)
        .bind(budget_record.details.to_string())
        .fetch_one(pool).await.expect("Failed to insert partner into DB"))
}

