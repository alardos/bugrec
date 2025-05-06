use sqlx::{Pool, Postgres, Acquire};

use super::entity::Category;

pub async fn get_all(pool: &Pool<Postgres>) -> Vec<Category> {
    let mut res = sqlx::query_as::<_,Category>("SELECT * FROM category")
        .fetch_all(pool).await.expect("something went wrong with the query");

    res.sort_by_cached_key(|t|t.name.to_string());
    res
}

pub async fn create(category: &Category, pool: &Pool<Postgres>) -> Option<Category> {
    sqlx::query_as::<_,Category>("INSERT INTO partner (name,special) VALUES($1,$2) RETURNING *;")
        .bind(&category.name)
        .bind(i16::from(&category.budget_type))
        .fetch_one(pool).await.ok()
}
