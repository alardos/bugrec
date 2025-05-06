use std::sync::Arc;

use http::{http_request::HttpRequest, json::{Json, JsonObj, JsonArr}, http_response::HttpResponse};
use log::error;


use crate::AppState;

use super::{tag::Tag, tag_repo};

pub async fn get_all(request: HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let tags = tag_repo::get_all(&app_state.db_pool).await;
    HttpResponse::ok_from(Json::from(tags))
}

pub async fn create(request: HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let Ok(tag_data) = serde_json::from_str::<Tag>(&request.body) else {
        error!("couldn't parse tag from:\n{}",request.body);
        return HttpResponse::bad();
    };

    match tag_repo::create_or_edit(&tag_data, &app_state.db_pool).await {
        Some(tag) => HttpResponse::ok_from(Json::from(tag)),
        None => HttpResponse::bad(),
    }
}

pub async fn set_parent(request: HttpRequest,app_state: Arc<AppState>) -> HttpResponse {
    let Some(tag_id) = request.get_param::<i64>("tagId") else {
        return HttpResponse::bad();
    };
    let Some(parent_id) = request.get_param::<i64>("parentId") else {
        return HttpResponse::bad();
    };

    match tag_repo::set_parent(tag_id, parent_id, &app_state.db_pool).await {
        None => HttpResponse::ok(),
        Some(err) => { error!("{err}"); HttpResponse::bad() },
    }
}

pub async fn prepared_list(request: HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let tags = tag_repo::get_all(&app_state.db_pool).await;
    let mut objs: Vec<JsonObj> = vec![];
    for tag in tags.iter() {
        let mut json = JsonObj::new();
        let children:Vec<Tag> = tags.iter().filter(|t|t.parent_id == tag.id.unwrap()).cloned().collect();
        let parent = tags.iter().find(|t|t.id.is_some_and(|x| x == tag.parent_id)).unwrap();
        json.push("id", Json::Num(tag.id.unwrap()));
        json.push("name", Json::Str(tag.name.to_string()));
        json.push("children", children);
        json.push("parent", parent.to_json());
        objs.push(json);
    }
    let arr = JsonArr::from(objs);
    HttpResponse::ok_from(arr)
}
