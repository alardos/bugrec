use std::fmt::Debug;
use std::future::Future;
use std::vec;

use log::{error, debug};
use sqlx::{Pool, Postgres, Executor, Transaction, PgExecutor, Acquire, Error, PgConnection, Connection};
use crate::tag::tag::Tag;
use crate::transfer;

use super::Partner;
use super::partner::PartnerWithTags;
use super::partner_alias::PartnerAlias;



pub(crate) async fn find(id: i64, pool: &Pool<Postgres>) -> Option<Partner> {
    sqlx::query_as::<_,Partner>("SELECT * FROM partner where id = $1")
        .bind(id)
        .fetch_all(pool).await
        .expect("something went wrong with the query")
        .pop()
}

pub async fn find_many<'e,E>(ids: &Vec<i64>, executor: E) -> Vec<Partner> 
    where E: sqlx::Executor<'e, Database = sqlx::Postgres> {

    sqlx::query_as::<_,Partner>("SELECT * FROM partner WHERE id = ANY($1)")
        .bind(&ids[..])
        .fetch_all(executor).await
        .expect("something went wrong with the query")
}

pub async fn find_many_alias(ids: Vec<i64>, pool: &Pool<Postgres>) -> Vec<PartnerAlias> {
    sqlx::query_as::<_,PartnerAlias>("SELECT * FROM partner_alias WHERE id IN $1")
        .bind(ids)
        .fetch_all(pool).await
        .expect("something went wrong with the query")
}

pub(crate) async fn find_by_name(pool: &Pool<Postgres>, name: &str) -> Option<Partner> {
    let result = sqlx::query_as::<_,Partner>("SELECT * FROM partner where name = $1")
        .bind(name)
        .fetch_all(pool).await.expect("something went wrong with the query")
        .pop();
    result
}

pub(crate) async fn find_alias_by_name<'e>(name: &str, acquarable: impl Acquire<'e, Database = Postgres>) -> Option<PartnerAlias> {

    let Ok(mut conn) = acquarable.acquire().await else { error!("couldn't acquire connection"); return None; };

    match sqlx::query_as::<_,PartnerAlias>("SELECT * FROM partner_alias where name = $1")
        .bind(name).fetch_one(&mut *conn).await {
            Ok(x) => Some(x),
            Err(e) => { error!("{e}"); None }
    }
}

pub async fn find_or_create(name: &str, pool: &Pool<Postgres>) -> Partner {
    let mut transaction = pool.begin().await.unwrap();
    let existing = find_by_name(pool, name).await;
    let res = match existing {
        Some(x) => x,
        None => create(&Partner::new(&name), &mut *transaction).await.unwrap(),
    };
    transaction.commit().await.unwrap();
    res
}

pub(crate) async fn get_all(pool: &Pool<Postgres>) -> Vec<Partner> {
    sqlx::query_as::<_,Partner>("SELECT * FROM partner")
        .fetch_all(pool).await
        .expect("something went wrong with the query")
}

pub(crate) async fn get_all_with_tags(pool: &Pool<Postgres>) -> Vec<PartnerWithTags> {
sqlx::query_as::<_,PartnerWithTags>("SELECT p.*, array_remove(array_agg(t.name), NULL) as tags FROM partner p LEFT JOIN tag_partner tp ON p.id = tp.partner_id LEFT JOIN TAG t ON t.id = tp.tag_id GROUP BY p.id")
        .fetch_all(pool).await
        .expect("something went wrong with the query")
}

fn block_on<F: Future>(future: F) -> F::Output {
    tokio::runtime::Runtime::new().unwrap().block_on(future)
}

pub async fn create<'e>(partner: &Partner, acquarable: impl Acquire<'e, Database = Postgres>) -> Option<Partner> {
    let Ok(mut transaction) = acquarable.begin().await else { error!("couldn't start transaction"); return None; };
    
    let new = sqlx::query_as::<_,Partner>("INSERT INTO partner (name,special) VALUES($1,$2) RETURNING *;")
        .bind(&partner.name)
        .bind(&partner.special)
        .fetch_one(&mut *transaction).await;

    match &new {
        Err(err) => { error!("{err}") },
        Ok(new) => { 
            if let Some(err) = sqlx::query("insert into partner_alias (name,partner_id) values($1,$2);")
                .bind(&partner.name)
                .bind(new.id)
                .execute(&mut *transaction).await.err() {
                    error!("{err}");
                } 
            },
    };

    transaction.commit().await.unwrap();
    new.ok()
}

pub async fn update<'e, E>(partner: &Partner, executor: E) -> Option<Partner> 
    where E: sqlx::Executor<'e, Database = sqlx::Postgres> {

    let new = sqlx::query_as::<_,Partner>("update partner set name=$1, special=$2 where id = $3 RETURNING *;")
        .bind(&partner.name)
        .bind(&partner.special)
        .bind(&partner.id)
        .fetch_one(executor).await;

    if let Err(err) = &new { error!("{err}"); };
    new.ok()
}

pub async fn update_alias<'e,E>(alias: PartnerAlias, executor: E) -> PartnerAlias 
    where E: sqlx::Executor<'e, Database = sqlx::Postgres> {

    sqlx::query_as::<_,PartnerAlias>("update partner_alias set name=$1, special=$2 where id = $3 RETURNING *;")
        .bind(&alias.name)
        .bind(&alias.partner_id)
        .bind(&alias.id)
        .fetch_one(executor).await.expect("Failed to insert partner into DB")
}

pub enum PartnerMergeErr { MixedSpecial, UnableToCreateNew, UnableToUpdateAliases, UnableToDeleteOldOnes, UnableToUpdateTransactions }
pub async fn merge(new_name: &str, old_partner_ids: Vec<i64>, pool: &Pool<Postgres>) -> Result<Partner, PartnerMergeErr> {
    let mut trasaction = pool.begin().await.unwrap();

    let olds = find_many(&old_partner_ids, &mut trasaction).await;

    let is_mixed:bool = olds.iter().any(|p|p.special) && olds.iter().any(|p|!p.special);
    if is_mixed { 
        return Err(PartnerMergeErr::MixedSpecial); 
    };

    let Some(new) = create(
        &Partner { id: -1, name: String::from(new_name), special: false },
        &mut trasaction, 
    ).await
    else { 
        return Err(PartnerMergeErr::UnableToCreateNew); 
    };


    let alias_update_err = sqlx::query("UPDATE partner_alias SET partner_id = $1 WHERE partner_id = ANY($2); ")
        .bind(new.id)
        .bind(&old_partner_ids[..])
        .execute(&mut trasaction).await.err(); 

    if let Some(err) = alias_update_err { 
        error!("{err}"); 
        return Err(PartnerMergeErr::UnableToUpdateAliases); 
    }


    let transfer_update_err = sqlx::query("UPDATE transfer SET partner_id = $1 WHERE partner_id = ANY($2); ")
        .bind(new.id)
        .bind(&old_partner_ids[..])
        .execute(&mut trasaction).await.err(); 

    if let Some(err) = transfer_update_err { 
        error!("{err}"); 
        return Err(PartnerMergeErr::UnableToUpdateTransactions); 
    }


    let old_partner_del_err = sqlx::query("DELETE FROM partner WHERE id = ANY($1)")
        .bind(&old_partner_ids[..])
        .execute(&mut trasaction).await.err();

    if let Some(err) = old_partner_del_err { 
        error!("{err}"); 
        return Err(PartnerMergeErr::UnableToDeleteOldOnes); 
    }


    if let Err(err) = trasaction.commit().await { 
        error!("{err}"); 
    }

    Ok(new)
}

pub async fn get_tags_for_partner<'e>(partner_id:i64, conn: &mut PgConnection ) -> Vec<Tag> {
    match sqlx::query_as::<_,Tag>("SELECT tag.* FROM tag JOIN tag_partner tp on tp.tag_id = tag.id join partner on partner.id = tp.partner_id where partner.id = $1")
        .bind(partner_id)
        .fetch_all(&mut *conn).await {
            Ok(tags) => tags,
            Err(e) => { error!("{e}"); vec![] }
        }
}

pub async fn assign_tag<'e>(partner_id: i64, tag_ids: Vec<i64>, acquirable: impl Acquire<'e, Database = Postgres> + Debug) -> Result<(), Error> {

    let mut transaction = acquirable.begin().await?;

    for tag_id in tag_ids {
        let result = sqlx::query("INSERT INTO tag_partner (tag_id, partner_id) values($1, $2) ")
            .bind(tag_id)
            .bind(partner_id)
            .execute(&mut *transaction).await;

        if let Err(err) = result {
            if !err.as_database_error().is_some_and(|e|e.message().contains("duplicate key")) { // if already exists throws an error and continues, this takes way less time then checking beforehand for all
                error!("{err}");
            } else {
                debug!("ignored error: {err}");
            }
        }
    }

    transaction.commit().await?;

    Ok(())


}
