use http::json::{JsonObj, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, postgres::PgRow, Row};


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum BudgetType { Income, Expense, Saving }

impl From<&BudgetType> for String {
    fn from(value: &BudgetType) -> Self {
        match value {
            BudgetType::Income => String::from("INCOME"),
            BudgetType::Expense => String::from("EXPENSE"),
            BudgetType::Saving => String::from("SAVING"),
        }
    }
}

impl BudgetRecord {
    pub fn to_json(value: &BudgetRecord) -> serde_json::Value {
        serde_json::json!({
            "id":value.id.to_string(),
            "date": value.date.to_string(),
            "budget_type":match value.budget_type {
                BudgetType::Income => "INCOME".to_string(),
                BudgetType::Expense => "EXPENSE".to_string(),
                BudgetType::Saving => "SAVING".to_string(),
            },
            "category":value.category_id.to_string(),
            "amount": value.amount.to_string(),
            "details": value.details.to_string()
        })
    }
}

impl From<&BudgetType> for i16 {
    fn from(value: &BudgetType) -> Self {
        match value {
            BudgetType::Income => 0,
            BudgetType::Expense => 1,
            BudgetType::Saving => 2,
        }
    }
}
impl From<i16> for BudgetType {
    fn from(value: i16) -> Self {
        match value {
            0 => BudgetType::Income,
            1 => BudgetType::Expense,
            2 => BudgetType::Saving,
            _ => panic!()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BudgetRecord {
    pub id: i64,
    pub date: i64,
    pub budget_type: BudgetType,
    pub category_id: i64,
    pub amount: f64,
    pub details: String
}

impl<'r> FromRow<'r, PgRow> for BudgetRecord {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let type_id: i16 = row.try_get("type")?;
        Ok(BudgetRecord {
            id: row.try_get("id")?,
            date: row.try_get("date")?,

            budget_type: BudgetType::from(type_id),
            category_id: row.try_get("category_id")?,
            amount: row.try_get("amount")?,
            details: row.try_get("details")?
        })
    }
}
impl From<Vec<BudgetRecord>> for Json {
    fn from(value: Vec<BudgetRecord>) -> Self {
        value.iter().map(|b|Json::from(b)).collect()
    }
}
impl From<&BudgetRecord> for Json {
    fn from(value: &BudgetRecord) -> Self {
        let mut json = JsonObj::new();

        json.push("id", Json::Str(value.id.to_string())); 
        json.push("date", Json::Str(value.date.to_string()));
        json.push("budget_type", Json::Str(match value.budget_type {
            BudgetType::Income => "INCOME".to_string(),
            BudgetType::Expense => "EXPENSE".to_string(),
            BudgetType::Saving => "SAVING".to_string(),
        }));
        json.push("category_id", Json::Str(value.category_id.to_string()));
        json.push("amount", Json::Str(value.amount.to_string()));
        json.push("details", Json::Str(value.details.to_string()));

        json.into()
    }
}
