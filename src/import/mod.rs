use core::panic;
use std::{dbg, sync::Arc};

use calamine::{Reader, Xlsx, open_workbook, DataType};
use http::{http_response::HttpResponse, http_request::HttpRequest};
use log::{info, error};
use sqlx::types::chrono::NaiveDateTime;

use crate::{transfer::{Transfer, transfer_repo}, AppState, partner::{partner_repo, partner::Partner}, way_of_payment::way_of_payment_repo, common::error::BError};

pub async fn transfers(request: HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let mut new_count = 0;
    for path in request.files {
        new_count += match import_from_file(app_state.clone(), path.to_str().unwrap()).await {
            Ok(x) => x.len(),
            Err(e) => {
                error!("{e}");
                return HttpResponse::bad();
            }
        }
    };
    HttpResponse::ok_from(format!("{{\"imported\":{}}}",new_count))
}

pub async fn import_from_file(app_state: Arc<AppState>, path: &str) -> Result<Vec<Transfer>, BError> {

    let mut excel: Xlsx<_> = open_workbook(path)?;
    let r = match excel.worksheet_range("Tranzakciók") {
        Some(Ok(range)) => range,
        Some(Err(e)) => return Err(e.into()),
        None => return Err(BError::from("sheet not found")),
    };
    info!("importing {{{}}} lines", r.height());
    let mut rows = r.rows();
    rows.next();
    let mut result: Vec<Transfer> = vec![];

    let mut virtual_balance = transfer_repo::get_balance(&app_state.db_pool).await; 

    for row in rows {
        info!("row={:?}, row[0]={:?}", row, row[0]);
        let partner = resolve_partner_by_its_alias(&row[4].to_string(), &app_state).await;
        let new_trasfer = Transfer { 
                id: -1, 
                auto_category: row[2].to_string(), 
                currency: row[11].to_string(),
                description: row[7].to_string(), 
                sum: to_num(&row[10]),
                original_balance: virtual_balance, 
                time: to_time(&row[0]), 
                partner_id: partner.id,
                way_of_payment_id: way_of_payment_repo::find_or_create(&app_state.db_pool, &row[2].to_string()).await.id, 
                assigned: false,
                note: String::new(),
            };

        virtual_balance = virtual_balance + new_trasfer.sum;
        result.push(new_trasfer);
    }

    let new_ones = transfer_repo::find_new_ones(&app_state.db_pool, result).await;
    dbg!(&new_ones);
    transfer_repo::create_all(new_ones, &app_state.db_pool).await.map_err(|e|e.into())
}

async fn resolve_partner_by_its_alias(name: &str, app_state: &AppState) -> Partner {
    let pool = &app_state.db_pool;

    // first search for an alias with this name
    if let Some(alias) = partner_repo::find_alias_by_name(name, pool).await {
        return partner_repo::find(alias.partner_id, pool).await.expect("inconsistent database records: partner alias referring nonexistent partner");
    }
    // second search for a partner with this name 
    if let Some(partner) = partner_repo::find_by_name(pool,name).await {
        return partner
    }
    // then search a partner with this name and if not fount create it
    let partner = partner_repo::create(&Partner::new(name), pool).await.unwrap(); 
    partner
}

fn to_time(data: &DataType) -> i64{
    dbg!(&data.to_string());
    match data {
        DataType::DateTime(x) => x.round() as i64,
        _ => NaiveDateTime::parse_from_str(&data.to_string(), "%Y-%m-%d %H:%M:%S").unwrap().timestamp()
    }
}

fn to_num(data: &DataType) -> f64 {
    match data {
        DataType::Int(x) => x.clone() as f64,
        DataType::Float(x) => x.clone(),
        DataType::String(x) => x.parse().unwrap(),
        _ => panic!(),
    }
}

