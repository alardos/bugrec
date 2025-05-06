use std::{sync::Arc, cmp::Reverse};

use http::{http_response::HttpResponse, json::Json, http_request::HttpRequest};
use log::warn;


use crate::AppState;

use super::transfer_repo;

pub async fn balance(app_state: Arc<AppState>) -> HttpResponse {
    HttpResponse::ok_from(transfer_repo::get_balance(&app_state.db_pool).await.to_string())
}

pub async fn get_all(app_state: Arc<AppState>) -> HttpResponse {
    let mut records = transfer_repo::get_all(&app_state.db_pool).await;
    records.sort_by_key(|r|Reverse(r.time));
    HttpResponse::ok_from::<Json>(records.into())
}

pub async fn get_by_partner(request: &HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let Some(partner_id) = request.get_param("id") else {
        return HttpResponse::bad_from("missing param: id")
    };

    let mut records = transfer_repo::get_all_by_partner(partner_id,&app_state.db_pool).await;
    records.sort_by_key(|r|Reverse(r.time));
    HttpResponse::ok_from::<Json>(records.into())
}

pub async fn get(request: &HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let Some(id) = request.get_param::<i64>("id") else { 
        warn!("missing param: id");
        return HttpResponse::bad_from("missing param: id");
    };

    match transfer_repo::find(&id, &app_state.db_pool).await {
        Some(transfer) => HttpResponse::ok_from(transfer.to_string()),
        None => HttpResponse::not_found()
    }
}
