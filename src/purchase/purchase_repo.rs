use sqlx::{Postgres, Pool};
use super::Purchase;


pub(crate) async fn get_all(pool: &Pool<Postgres>) -> Vec<Purchase> {
    sqlx::query_as::<_,Purchase>("SELECT * FROM purchase")
        .fetch_all(pool).await.expect("something went wrong with the query")
}

pub async fn save_all(purchases: Vec<Purchase>, pool: &Pool<Postgres>) {
    for purchase in purchases {
        sqlx::query("INSERT INTO purchase (item_id, time, partner_id, sum, transfer_id) VALUES($1,$2,$3,$4,$5);")
            .bind(purchase.item_id)
            .bind(purchase.time)
            .bind(purchase.partner_id)
            .bind(purchase.sum)
            .bind(purchase.transfer_id)
            .execute(pool).await.unwrap();
    }
}

pub async fn create(purchase: &Purchase, pool: &Pool<Postgres>) -> Option<Purchase> {
    Some(sqlx::query_as::<_,Purchase>("INSERT INTO purchase (item_id,time,partner_id,sum,transfer_id) VALUES($1,$2,$3,$4,$5) RETURNING *;")
        .bind(purchase.item_id)
        .bind(purchase.time)
        .bind(purchase.partner_id)
        .bind(purchase.sum)
        .bind(purchase.transfer_id)
        .fetch_one(pool).await.expect("Failed to insert partner into DB"))
}
