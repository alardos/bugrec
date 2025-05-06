use std::{sync::Arc, fs};

use http::{http_response::HttpResponse, json::{Json, JsonObj}, http_request::HttpRequest};
use log::{error, debug};
use serde::{Serialize, Deserialize};
use sqlx::types::chrono::{NaiveDateTime, NaiveDate, DateTime, Utc, NaiveTime};
use tera::Tera;

use crate::{AppState, category};

use super::{budget_record_repo, budget_record::{BudgetRecord, BudgetType::{Income, Expense, Saving}}};

pub async fn tracking_page(app_state: Arc<AppState>) -> HttpResponse {
    let categories = category::repo::get_all(&app_state.db_pool).await;
    let types = vec!["INCOME","EXPENSE","SAVING"];
    let budget_records = get_all_s(app_state.clone()).await;
    let temp = String::from_utf8(fs::read("template/tracking.html").unwrap()).unwrap();
    let mut tera = Tera::new("templates/**/*.html").unwrap();
    let mut context = tera::Context::new();
    context.insert("budget_records",&budget_records);
    context.insert("category_options",&categories);
    context.insert("type_options", &types);
    debug!("{:#?}",&context);
    return HttpResponse::ok_from(tera.render_str(&temp,&context).unwrap())

}
#[derive(Debug,Serialize, Deserialize)]
struct SimpleBudgetRecord {
    pub id: Option<i64>,
    pub date: String,
    pub budget_type: String,
    pub category: i64,
    pub amount: i64,
    pub details: String
}
impl SimpleBudgetRecord {
    fn from(value: Vec<SimpleBudgetRecord>) -> Self {
        value.into_iter().map(|r| {
            let mut json = JsonObj::new();

            json.push("id", Json::Str(r.id.unwrap().to_string())); 
            json.push("date", Json::Str(r.date.to_string()));
            json.push("budget_type", Json::Str(String::from(&r.budget_type)));
            json.push("category", Json::Str(r.category.to_string()));
            json.push("amount", Json::Str(r.amount.to_string()));
            json.push("details", Json::Str(r.details.to_string()));

            json.into()

        }).collect()
    }
}
async fn get_all_s(app_state: Arc<AppState>) -> Vec<SimpleBudgetRecord> {
    let categories = category::repo::get_all(&app_state.db_pool).await;
    let records = budget_record_repo::get_all(&app_state.db_pool).await;
    records.into_iter().map(|r| {
        let nd = NaiveDateTime::from_timestamp_millis(r.date*1000).unwrap();
        let datetime: DateTime<Utc> = DateTime::from_utc(nd, Utc);
        
        SimpleBudgetRecord {
            id:Some(r.id),
            date:datetime.format("%Y-%m-%d").to_string(),
            budget_type:String::from(&r.budget_type),
            category: r.category_id,
            amount:r.amount as i64,
            details:r.details
        }
    }).collect()
}
pub async fn get_all(app_state: Arc<AppState>) -> HttpResponse {
    let categories = category::repo::get_all(&app_state.db_pool).await;
    let records = budget_record_repo::get_all(&app_state.db_pool).await;
    let res = records.into_iter().map(|r| {
        let mut j = Json::from(&r);
        if let Json::Obj(j) = &mut j {
            j.insert("category".to_string(),Json::Str(categories.iter().find(|t|t.id == r.category_id).unwrap().name.to_string()));
        }
        return j;
    }).collect::<Json>();
    return HttpResponse::ok_from(res);
}
pub async fn create(request: HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    debug!("create budget record");
    let Ok(data) = serde_json::from_str::<SimpleBudgetRecord>(&request.body) else {
        error!("couldn't parse budget_record from:\n{}",request.body);
        return HttpResponse::bad();
    };

    match budget_record_repo::create(&BudgetRecord::from(data), &app_state.db_pool).await {
        Some(budget_record) => HttpResponse::ok_from(Json::from(&budget_record)),
        None => HttpResponse::bad(),
    }
}

impl From<SimpleBudgetRecord> for BudgetRecord {
    fn from(value: SimpleBudgetRecord) -> Self {
        debug!("{:#?}",&value);
        BudgetRecord {
            id: match value.id {
                Some(id) => id,
                None => -1
            },
            date: NaiveDate::parse_from_str(&value.date, "%Y-%m-%d").unwrap().and_time(NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap()).timestamp(),
            budget_type: match value.budget_type.as_str() {
                "INCOME" => Income,
                "EXPENSE" => Expense,
                "SAVING" => Saving,
                _ => panic!()
            },
            category_id: value.category,
            amount: value.amount as f64,
            details: value.details,
        }
    }
}
