use std::vec;

use log::{error, debug};
use sqlx::{Pool, Postgres};
use crate::tag::tag::Tag;

use super::item::Item;

pub(crate) async fn get(id: &i64, pool: &Pool<Postgres>) -> Option<Item> {
    sqlx::query_as::<_, Item>("SELECT * FROM item where id = $1")
        .bind(id)
        .fetch_all(pool).await
        .expect("something went wrong with the query")
        .pop()
}

pub async fn create(item: &Item, pool: &Pool<Postgres>) -> Option<Item> {
    Some(sqlx::query_as::<_,Item>("INSERT INTO item (bar_code,name,picture_url) VALUES($1,$2,$3) RETURNING *;")
        .bind(&item.bar_code)
        .bind(&item.name)
        .bind(&item.picture_url.clone().unwrap_or("".to_string()))
        .fetch_one(pool).await.expect("Failed to insert partner into DB"))
}

pub async fn get_tags_for_item(item_id: i64, pool: &Pool<Postgres>) -> Vec<Tag> {
    match sqlx::query_as::<_,Tag>("SELECT tag.* FROM tag JOIN tag_item ti on ti.tag_id = tag.id join item on item.id = ti.item_id where item.id = $1")
        .bind(item_id)
        .fetch_all(pool).await {
            Ok(tags) => tags,
            Err(e) => { error!("{e}"); vec![] }
        }
}

pub async fn assign_tag(item_id: i64, tag_ids: Vec<i64>, pool: &Pool<Postgres>) {
    for tag_id in tag_ids {
        let result =sqlx::query("INSERT INTO tag_item (tag_id, item_id) values($1, $2) ")
            .bind(tag_id).bind(item_id).execute(pool).await;
        if let Err(err) = result {
            if !err.as_database_error().unwrap().message().contains("duplicate key") { // if already exists throws an error and continues, this takes way less time then checking beforehand for all
                error!("{err}");
            } else {
                debug!("ignored error: {err}");
            }
        }
    }
}

pub(crate) async fn get_all(pool: &Pool<Postgres>) -> Vec<Item> {
    sqlx::query_as::<_,Item>("SELECT * FROM item")
        .fetch_all(pool).await.expect("something went wrong with the query")
}