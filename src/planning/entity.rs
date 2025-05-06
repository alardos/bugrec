use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Plan {
    pub year: i16,
    pub month: i16,
    pub category_id: i64,
    pub amount: f64,
}
