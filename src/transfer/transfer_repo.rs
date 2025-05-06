use sqlx::Acquire;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use crate::partner::partner_repo;

use super::Transfer;



pub(crate) async fn find(id: &i64, pool: &Pool<Postgres>) -> Option<Transfer> {
    sqlx::query_as::<_, Transfer>("SELECT * FROM transfer where id = $1")
        .bind(id)
        .fetch_all(pool).await
        .expect("something went wrong with the query")
        .pop()
}

pub(crate) async fn get_all(pool: &Pool<Postgres>) -> Vec<Transfer> {
    sqlx::query_as::<_, Transfer>("SELECT * FROM transfer")
        .fetch_all(pool).await
        .expect("something went wrong with the query")
}

pub(crate) async fn get_all_by_partner(partner_id: i64, pool: &Pool<Postgres>) -> Vec<Transfer> {
    sqlx::query_as::<_, Transfer>("SELECT * FROM transfer WHERE partner_id = $1")
        .bind(&partner_id)
        .fetch_all(pool).await
        .expect("something went wrong with the query")
}

pub async fn create_all<'e>( records: Vec<Transfer>, acquarable: impl Acquire<'e, Database = Postgres>) -> Result<Vec<Transfer>,Error> {
    let mut transaction = acquarable.begin().await?;

    let mut created = vec![];
    for record in records {
        let transfer = sqlx::query_as::<_,Transfer>("INSERT INTO transfer (auto_category, currency, description, sum, time, partner_id, way_of_payment_id, original_balance, assigned, note) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) RETURNING *;")
            .bind(&record.auto_category)
            .bind(&record.currency)
            .bind(&record.description)
            .bind(&record.sum)
            .bind(&record.time)
            .bind(&record.partner_id)
            .bind(&record.way_of_payment_id)
            .bind(&record.original_balance)
            .bind(&record.assigned)
            .bind(&record.note)
            .fetch_one(&mut transaction).await?;
        let tags = partner_repo::get_tags_for_partner(transfer.partner_id, &mut transaction).await;
        for tag in tags {
            sqlx::query("insert into tag_transfer (tag_id,transfer_id) values($1,$2);").bind(tag.id).bind(transfer.id).execute(&mut transaction).await?;
        } 

        created.push(transfer);
    }

    transaction.commit().await?;
    Ok(created)
}

pub async fn find_new_ones(pool: &Pool<Postgres>, news: Vec<Transfer>) -> Vec<Transfer> {
    let olds = get_all(pool).await;
    
    news.into_iter().filter(|new| !olds.iter().find(|old| matches_in_time(new, old)).is_some()).collect()
}

const ONE_SEC_IN_MILLIS: i64 = 1000;

fn matches_in_time(a: &Transfer, b: &Transfer) -> bool {
    (a.time - b.time).abs() < ONE_SEC_IN_MILLIS 
}

pub(crate) async fn update_all<'e>(records: Vec<Transfer>, acquarable: impl Acquire<'e, Database = Postgres>)-> Result<(),Error> {
    let mut transfer = acquarable.begin().await?;
    for record in records {
        sqlx::query("UPDATE transfer SET auto_category = $1, currency = $2, description = $3, sum = $4, time = $5, partner_id = $6, way_of_payment_id = $7, original_balance = $8 where id = $9")
            .bind(&record.auto_category)
            .bind(&record.currency)
            .bind(&record.description)
            .bind(&record.sum)
            .bind(&record.time)
            .bind(&record.partner_id)
            .bind(&record.way_of_payment_id)
            .bind(&record.original_balance)
            .bind(&record.id).execute(&mut *transfer).await?;
    }

    Ok(())
}

pub async fn get_balance(pool: &Pool<Postgres>) -> f64 {
    get_all(pool).await.iter().map(|r| r.sum).sum()
}

pub async fn fix_original_balances(pool: &Pool<Postgres>) {
    let mut moving_balance:f64 = 0.0;
    let mut records = get_all(pool).await;
    records.sort_by_key(|r|r.id);
    for record in &mut records {
        record.original_balance = moving_balance;
        moving_balance = moving_balance + record.sum;
    }

    update_all(records, pool).await;
}
