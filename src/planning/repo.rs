use sqlx::{Pool, Postgres};

use super::entity::Plan;

pub async fn get_all(pool: &Pool<Postgres>) -> Vec<Plan> {
    let mut res = sqlx::query_as::<_,Plan>("SELECT * FROM plan")
        .fetch_all(pool).await.expect("something went wrong with the query");

    res
}

pub async fn get_all_for_year(year: i16, pool: &Pool<Postgres>) -> Vec<Plan> {
    sqlx::query_as::<_,Plan>("SELECT * FROM plan where year = $1")
        .bind(year)
        .fetch_all(pool).await.expect("something went wrong with the query")
}

pub async fn update(plan: Plan, pool: &Pool<Postgres>) -> Plan {
    sqlx::query_as::<_,Plan>("UPDATE plan SET amount = $1 WHERE year = $2 AND month = $3 AND category_id = $4 RETURNING *")
        .bind(plan.amount)
        .bind(plan.year)
        .bind(plan.month)
        .bind(plan.category_id)
        .fetch_one(pool).await.unwrap()
}
