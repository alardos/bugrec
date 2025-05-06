use std::sync::Arc;

use http::{http_response::HttpResponse, http_request::HttpRequest, json::Json};
use log::{warn, debug};

use crate::AppState;

use super::{Purchase, purchase_repo};

pub async fn add_purchase(request: &HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    match serde_json::from_str::<Purchase>(&request.body) {
        Err(err) => {
            warn!("couldn't parse purchase from {:#?}, {err}", request.body);
            return HttpResponse::bad()
        },
        Ok(purchase_data) => {
            match purchase_repo::create(&purchase_data, &app_state.db_pool).await {
                Some(purchase) => HttpResponse::ok_from(purchase.to_string()),
                None => HttpResponse::bad(),
            }
        }
    }
}

pub async fn get_all_purchases(request: &HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let purchases: Json = purchase_repo::get_all(&app_state.db_pool).await.into();
    debug!("purchases:\n{}", purchases.to_string());
    HttpResponse::ok_from(purchases)
}
