use std::{fs, sync::Arc};

use http::{http_request::HttpRequest, http_response::HttpResponse};
use log::debug;
use serde::Serialize;
use tera::Tera;

use crate::{category::{entity::Category, self}, AppState, planning, budget_record::budget_record::BudgetType};

use super::{entity::Plan, repo};

#[derive(Serialize)]
struct CategoryRow {
    id: i64,
    name: String,
    months: Months,
}

impl CategoryRow {
    fn convert(plans: &Vec<Plan>, category:Vec<&Category>) -> Vec<CategoryRow> {
        category.iter().map(|c| CategoryRow {
            id: c.id,
            name: c.name.to_string(),
            months: Months {
                jan: plans.iter().find(|p| p.category_id==c.id && p.month==1 ).map(|p|p.amount).unwrap_or(0.0),
                feb: plans.iter().find(|p| p.category_id==c.id && p.month==2 ).map(|p|p.amount).unwrap_or(0.0),
                mar: plans.iter().find(|p| p.category_id==c.id && p.month==3 ).map(|p|p.amount).unwrap_or(0.0),
                apr: plans.iter().find(|p| p.category_id==c.id && p.month==4 ).map(|p|p.amount).unwrap_or(0.0),
                may: plans.iter().find(|p| p.category_id==c.id && p.month==5 ).map(|p|p.amount).unwrap_or(0.0),
                jun: plans.iter().find(|p| p.category_id==c.id && p.month==6 ).map(|p|p.amount).unwrap_or(0.0),
                jul: plans.iter().find(|p| p.category_id==c.id && p.month==7 ).map(|p|p.amount).unwrap_or(0.0),
                aug: plans.iter().find(|p| p.category_id==c.id && p.month==8 ).map(|p|p.amount).unwrap_or(0.0),
                sep: plans.iter().find(|p| p.category_id==c.id && p.month==9 ).map(|p|p.amount).unwrap_or(0.0),
                oct: plans.iter().find(|p| p.category_id==c.id && p.month==10).map(|p|p.amount).unwrap_or(0.0),
                nov: plans.iter().find(|p| p.category_id==c.id && p.month==11).map(|p|p.amount).unwrap_or(0.0),
                dec: plans.iter().find(|p| p.category_id==c.id && p.month==12).map(|p|p.amount).unwrap_or(0.0),
            }
        }).collect()
    }
}

#[derive(Serialize)]
struct Months { jan:f64, feb:f64, mar:f64, apr:f64, may:f64, jun:f64, jul:f64, aug:f64, sep:f64, oct:f64, nov:f64, dec:f64, }

pub async fn planning_page(request: HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let year: i16 = request.get_param("year").unwrap();
    let plans = planning::repo::get_all_for_year(year, &app_state.db_pool).await; 
    let categories = category::repo::get_all(&app_state.db_pool).await;

    let income_rows = CategoryRow::convert(&plans, categories.iter().filter(|c| c.budget_type == BudgetType::Income).collect());
    let expense_rows = CategoryRow::convert(&plans, categories.iter().filter(|c| c.budget_type == BudgetType::Expense).collect());
    let saving_rows = CategoryRow::convert(&plans, categories.iter().filter(|c| c.budget_type == BudgetType::Saving).collect());

    let temp = String::from_utf8(fs::read("template/planning.html").unwrap()).unwrap();
    let mut tera = Tera::new("templates/**/*.html").unwrap();
    let mut context = tera::Context::new();
    context.insert("income_rows",&income_rows);
    context.insert("expense_rows",&expense_rows);
    context.insert("saving_rows",&saving_rows);
    debug!("{:#?}",&context);
    return HttpResponse::ok_from(tera.render_str(&temp,&context).unwrap())
}

pub async fn update(request: HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let year = request.get_param("year").unwrap();
    let month = request.get_param("month_id").unwrap();
    let category_id = request.get_param("category_id").unwrap();
    let amount = request.get_param("amount").unwrap();

    repo::update(Plan { year, month, category_id, amount }, &app_state.db_pool).await;
    return HttpResponse::ok();
}
