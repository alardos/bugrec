use std::{sync::Arc};
use http::{http_response::HttpResponse, json::{JsonArr, Json}, http_request::HttpRequest};
use log::{warn, error};
use serde::Deserialize;
use crate::{AppState, partner::partner_repo::PartnerMergeErr::MixedSpecial, common::{self, err_codes}, tag::tag_repo};
use super::partner_repo;


pub async fn get_all(app_state: Arc<AppState>) -> HttpResponse { 
    let partners = partner_repo::get_all_with_tags(&app_state.db_pool).await;
    HttpResponse::ok_from::<JsonArr>(JsonArr::from(partners))
}

pub async fn get(request: &HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let Some(id) = request.get_param::<i64>("id") else { 
        warn!("param {{id}} couldn't be parsed from request");
        return HttpResponse::err_from("missing param"); 
    };

    let mut conn = app_state.db_pool.begin().await.unwrap();
    match partner_repo::find(id, &app_state.db_pool).await {
        Some(partner) => {
            let tags = partner_repo::get_tags_for_partner(partner.id, &mut conn).await;
            let mut json = partner.to_json();
            match &mut json {
                Json::Obj(json) => {
                    json.insert(String::from("implicitTags"), tag_repo::implicit_from_explicit(&tags, &app_state.db_pool).await.into());
                    json.insert(String::from("explicitTags"), tags.into());
                }, 
                _ => error!("Unexpectedly the json varian is not an object. Leave tags out of response")
            }
            HttpResponse::ok_from(json)
        },
        None => HttpResponse::not_found(),
    }
}

pub async fn merge(request:HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    #[derive(Deserialize)]
    struct MergeBodyPrototype { old_partner_ids: Vec<i64>, new_name: String }

    let merge_params:MergeBodyPrototype = match serde_json::from_str(&request.body) {
        Ok(val) => val,
        Err(err) => {
            error!("{err}");
            return HttpResponse::bad();
        }
    };

    match partner_repo::merge(&merge_params.new_name, merge_params.old_partner_ids, &app_state.db_pool).await {
        Ok(new) => HttpResponse::ok_from(new.to_json()),
        Err(MixedSpecial) => HttpResponse::bad_from(err_codes::PARTNER_MERGE_MIXED_MERGE),
        Err(_) => HttpResponse::bad(),
    }
}

pub async fn assign_tag_to_partner(request: HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let Some(partner_id) = request.get_param::<i64>("partner-id") else {
        return HttpResponse::bad();
    };
    let Some(tag_id) = request.get_param::<i64>("tag-id") else {
        return HttpResponse::bad();
    };
    partner_repo::assign_tag(partner_id, vec![tag_id], &app_state.db_pool).await;
    HttpResponse::ok()
}
