use std::sync::Arc;

use http::{http_request::HttpRequest, http_response::HttpResponse};
use log::error;
use serde::Deserialize;
use sqlx::types::chrono;

use crate::{AppState, budget_record::budget_record::BudgetType, category::repo, planning};

use super::entity::Category;

#[derive(Deserialize)]
struct Name {
    name: String
}

pub async fn create(request: HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let budget_type = match request.get_param::<String>("type").unwrap().as_str() {
        "income" => BudgetType::Income,
        "expense" => BudgetType::Expense,
        "saving" => BudgetType::Saving,
        _ => panic!()
    };

    let name = serde_json::from_str::<Name>(&request.body).unwrap().name;

    repo::create(&Category {id: -1, name, budget_type}, &app_state.db_pool).await;
    return HttpResponse::ok();

}
