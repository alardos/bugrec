use std::sync::Arc;

use http::{http_response::HttpResponse, json::Json, http_request::HttpRequest};
use log::error;


use crate::{AppState, tag::tag_repo};

use super::{item_repo, item::Item};



pub async fn get_all(request: &HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let json: Json = item_repo::get_all(&app_state.db_pool).await.into();
    HttpResponse::ok_from(json)
}

pub async fn get(request: HttpRequest,app_state: Arc<AppState>) -> HttpResponse {
    let Some(id) = request.get_param::<i64>("id") else { return HttpResponse::bad(); };
    match item_repo::get(&id, &app_state.db_pool).await {
        Some(item) => {
            let tags = item_repo::get_tags_for_item(item.id.unwrap(), &app_state.db_pool).await;
            let mut json = item.to_json();
            match &mut json {
                Json::Obj(json) => {
                    json.insert(String::from("implicitTags"), tag_repo::implicit_from_explicit(&tags, &app_state.db_pool).await.into());
                    json.insert(String::from("explicitTags"), tags.into());
                }, 
                _ => error!("Unexpectedly the json varian is not an object. Leave tags out of response")
            }
            HttpResponse::ok_from(json)
        }
        None => HttpResponse::not_found(),
    }
}

pub async fn create(request: &HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let item_data: Item = serde_json::from_str(&request.body).expect("couldn't parse");
    match item_repo::create(&item_data, &app_state.db_pool).await {
        Some(item) => HttpResponse::ok_from(item.to_string()),
        None => HttpResponse::bad(),
    }
}

pub async fn assign_tag_to_item(request: HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let Some(item_id) = request.get_param::<i64>("item-id") else {
        return HttpResponse::bad();
    };
    let Some(tag_id) = request.get_param::<i64>("tag-id") else {
        return HttpResponse::bad();
    };
    item_repo::assign_tag(item_id, vec![tag_id], &app_state.db_pool).await;
    HttpResponse::ok()
}
