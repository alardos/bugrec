mod http_server;
mod tag;
mod import;
mod item;
mod purchase;
mod partner;
mod common;
mod transfer;
mod way_of_payment;
#[cfg(test)]
mod tests;
pub mod budget_record;
pub mod planning;
pub mod category;

use env_logger;
use log::{error, info};
use std::{time::Duration, fs::File};
use sqlx::{Pool, Postgres, postgres::{PgPoolOptions, PgConnectOptions}, ConnectOptions};
use tokio;

const CONNECTION_PATH: &str = "postgres://postgres:1234@localhost:5433/postgres";

#[tokio::main]
async fn main() {
    let target = Box::new(File::create("./log.txt").expect("Can't create file"));
    // env_logger::builder().filter_level(log::LevelFilter::Debug).target(env_logger::Target::Pipe(target)).init();
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();
    info!("starting finory-diy...");
    //let _ = handler().await;

    let mut options = PgConnectOptions::new().host("localhost").port(5432).database("postgres").username("postgres").password("1234");
    options.disable_statement_logging();
    let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .acquire_timeout(Duration::from_secs(3))
                    .connect_with(options).await; 

    let Ok(pool) = pool else {
        error!("couldn't estabilish connection to the db, exiting...");
        return;
    };
    
    http_server::start(AppState{ db_pool: pool }).await;
    // repository::transfer_repo::fix_original_balances(&pool).await;
    // if let Err(e) = import::import_from_file(&AppState { db_pool: pool }, "resources/imports/transactions.xlsx").await { error!("{e}"); }
}

pub struct AppState {
    db_pool: Pool<Postgres>
}
