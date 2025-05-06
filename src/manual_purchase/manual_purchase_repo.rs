use sqlx::{Postgres, Pool};
use crate::repository::common_repo::get_pool;


pub(crate) async fn get_all(pool: &Pool<Postgres>) -> Vec<ManualPurchase> {
    sqlx::query_as::<_, ManualPurchase>("SELECT * FROM manual_purchase")
        .fetch_all(pool).await
        .expect("something went wrong with the query")
}

pub async fn save_all(manual_purchases: Vec<ManualPurchase>) {
    match &get_pool().await {
        Err(..) => (),
        Ok(pool) => {
            for purchase in manual_purchases {
                sqlx::query("INSERT INTO manual_purchase (time,price,partner_id,item_id,latitude,longitude) VALUES($1,$2,$3,$4,$5,$6);")
                    .bind(purchase.time)
                    .bind(purchase.price)
                    .bind(purchase.partner_id)
                    .bind(purchase.item_id)
                    .bind(purchase.latitude)
                    .bind(purchase.longitude)
                    .execute(pool).await.unwrap();
            }
        }
    }
}


pub async fn clear() {
    match &get_pool().await {
        Err(..) => (),
        Ok(pool) => _ = sqlx::query("DELETE FROM manual_purchase").execute(pool).await.unwrap(),
    }
}


impl Clone for ManualPurchase {
    fn clone(&self) -> Self {
        ManualPurchase { ..*self }
    }
}
