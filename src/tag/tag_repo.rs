use log::error;
use serde::Deserialize;
use sqlx::{Pool, Postgres, Error};


use super::tag::Tag;


pub(crate) async fn find(id: &i64, pool: &Pool<Postgres>) -> Option<Tag> {
    sqlx::query_as::<_, Tag>("SELECT * FROM tag where id = $1")
        .bind(id)
        .fetch_all(pool).await
        .expect("something went wrong with the query")
        .pop()
}

pub(crate) async fn find_by_name(name: &str, pool: &Pool<Postgres>) -> Option<Tag> {
    sqlx::query_as::<_, Tag>("SELECT * FROM tag where name = $1")
        .bind(name)
        .fetch_all(pool).await
        .expect("something went wrong with the query")
        .pop()
}

pub async fn implicit_from_explicit(explicits: &Vec<Tag>, pool: &Pool<Postgres>) -> Vec<Tag> {
    let explicit_ids:Vec<i64> = explicits.iter().filter(|t|t.id.is_some()).map(|t|t.id.unwrap()).collect();
    let all = get_all(pool).await;
    let mut discovered:Vec<i64> = explicit_ids.clone();
    let mut checked:Vec<i64> = vec![];

    while discovered.len() > 0 {
        let next = discovered.pop();
        let next = all.iter().find(|t|t.id == next).unwrap();
        let new = next.parent_id;
        if !discovered.contains(&new) && !checked.contains(&new) {
            discovered.push(new);
        }
        checked.push(next.id.unwrap());
    }

    all.into_iter().filter(|t|checked.contains(&t.id.unwrap()) && !explicit_ids.contains(&t.id.unwrap())).collect()
}


pub async fn create_or_edit(tag: &Tag, pool: &Pool<Postgres>) -> Option<Tag> {
    let tag = match find_by_name(&tag.name, pool).await {
        Some(existing) => sqlx::query_as::<_,Tag>("UPDATE tag SET parent_id = $1 WHERE id = $2 RETURNING *;")
            .bind(&tag.parent_id)
            .bind(existing.id)
            .fetch_one(pool).await,
        
        None => sqlx::query_as::<_,Tag>("INSERT INTO tag (name, parent_id) VALUES($1,$2) RETURNING *;")
            .bind(&tag.name.trim())
            .bind(&tag.parent_id)
            .fetch_one(pool).await
    };

    match tag {
        Ok(tag) => Some(tag),
        Err(e) => { error!("{e}"); None }
    }
}

pub async fn get_all(pool: &Pool<Postgres>) -> Vec<Tag> {
    let mut res = sqlx::query_as::<_,Tag>("SELECT * FROM tag")
        .fetch_all(pool).await.expect("something went wrong with the query");

    res.sort_by_cached_key(|t|t.name.to_string());
    res
}

pub async fn set_parent(tag_id: i64, parent_id: i64, pool: &Pool<Postgres>) -> Option<Error> {
    sqlx::query("UPDATE tag SET parent_id = $2 WHERE id = $1").bind(tag_id).bind(parent_id).execute(pool).await.err()
}
