use serde::{Serialize, Deserialize};
use sqlx::{FromRow, postgres::PgRow, Row};

use crate::budget_record::budget_record::BudgetType;

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub budget_type: BudgetType,
}

impl<'r> FromRow<'r, PgRow> for Category {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let type_id: i16 = row.try_get("type")?;
        Ok(Category {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            budget_type: BudgetType::from(type_id),
        })
    }
}
