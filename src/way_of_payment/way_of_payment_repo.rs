use std::dbg;

use sqlx::{Postgres, Pool};

use super::WayOfPayment;



pub async fn find(id: i64, pool: &Pool<Postgres>) -> Option<WayOfPayment> {
    sqlx::query_as::<_,WayOfPayment>("SELECT * FROM way_of_payment where id = $1")
        .bind(id)
        .fetch_all(pool).await.expect("something went wrong with the query")
        .pop()
}

pub async fn find_or_create(pool: &Pool<Postgres>, name: &String) -> WayOfPayment {
    dbg!(name);
    match sqlx::query_as::<_,WayOfPayment>("SELECT * FROM way_of_payment where name = $1")
        .bind(name)
        .fetch_all(pool).await.expect("something went wrong with the query")
        .pop() {
            Some(x) => x,
            None => create(pool, WayOfPayment::new(name)).await.unwrap(),
        }
}

pub async fn create(pool: &Pool<Postgres>, way_of_payment: WayOfPayment) -> Option<WayOfPayment> {
    sqlx::query_as::<_,WayOfPayment>("INSERT INTO way_of_payment (name) VALUES($1) RETURNING *;")
        .bind(&way_of_payment.name)
        .fetch_one(pool).await.ok()
}

impl WayOfPayment {
    fn new(name: &String) -> Self {
        WayOfPayment {
            name: name.to_string(),
            id: -1
        }
    }
}
