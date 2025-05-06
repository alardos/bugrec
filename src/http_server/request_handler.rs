use std::io::Read;
use std::{net::TcpStream, fs};
use std::sync::Arc;
use http::http_response::HttpResponse;
use log::{error, info, debug};
use crate::budget_record::budget_record_controller;
use crate::common::common_controller;
use crate::{import, planning};
use crate::item::item_controller;
use crate::partner::partner_controller;
use crate::purchase::purchase_controller;
use crate::tag::tag_controller;
use crate::transfer::transfer_controller;
use crate::AppState;
use http::http_request::HttpRequest;

pub async fn handle_request(mut stream: TcpStream, app_state: Arc<AppState>) {

    let request = match HttpRequest::parse(&mut stream) {
        Ok(request) => request,
        Err(e) => {
            error!("{e}");
            HttpResponse::err().write(stream);
            return; 
        }
    };

    info!("uri: {:?}", request.uri);
    debug!("{:#?}",request);
    let response = match request.uri.as_str() {
        "/api/partners/get" => partner_controller::get(&request, app_state).await,
        "/api/partners" => partner_controller::get_all(app_state).await,
        "/api/partners/merge" => partner_controller::merge(request, app_state).await,
        "/api/partners/assign-tag" => partner_controller::assign_tag_to_partner(request,app_state).await,

        "/api/item/create" => item_controller::create(&request, app_state).await,
        "/api/item/get-all" => item_controller::get_all(&request, app_state).await,
        "/api/item/get" => item_controller::get(request, app_state).await,
        "/api/item/assign-tag" => item_controller::assign_tag_to_item(request,app_state).await,

        "/api/purchase/create" => purchase_controller::add_purchase(&request, app_state).await,
        "/api/purchase/get-all" => purchase_controller::get_all_purchases(&request, app_state).await,

        "/api/tag/get-all" => tag_controller::get_all(request,app_state).await,
        "/api/tag/prepared-list" => tag_controller::prepared_list(request,app_state).await,
        "/api/tag/create" => tag_controller::create(request, app_state).await,
        "/api/tag/set-parent" => tag_controller::set_parent(request, app_state).await,

        "/api/transfer/import" => import::transfers(request,app_state).await,
        "/api/records" => transfer_controller::get_all(app_state).await,
        "/api/balance" => transfer_controller::balance(app_state).await,
        "/api/records/get" => transfer_controller::get(&request, app_state).await,
        "/api/records/by_partner" => transfer_controller::get_by_partner(&request, app_state).await,

        "/api/budget-record/get-all" => budget_record_controller::get_all(app_state).await,
        "/api/budget-record/add" => budget_record_controller::create(request,app_state).await,

        "/api/planning/update" => planning::controller::update(request,app_state).await,

        "/api/img" => common_controller::resolve_img(&request, app_state),
        // "/api/push") => controller::push(&request)?,

        "/record" => read_template("record_details.html"),
        "/partner" => read_template("partner_details.html"),
        "/partners" => read_template("partners.html"),
        "/add-item" => read_template("add_item.html"),
        "/main.js" => read_template("main.js"),
        "/items" => read_template("items.html"),
        "/item" => read_template("item_details.html"),
        "/add-purchase" => read_template("add_purchase.html"),
        "/purchases" => read_template("purchases.html"),
        "/tags" => read_template("tags.html"),
        "/nav" => read_template("nav.html"),
        "/tracking" => budget_record_controller::tracking_page(app_state).await,
        "/planning" => planning::controller::planning_page(request, app_state).await,
        "/" => read_template("index.html"),
        _ => read_not_found_page_template(),
    };
    debug!("write back onto the tcp stream");
    response.write(stream);
}

fn read_not_found_page_template() -> HttpResponse {
    debug!("read_not_found_page_template()");
    let path = "template/404.html";
    match fs::read(path) {
        Ok(content) => HttpResponse::not_found_from(content),
        Err(_) => {
            error!("couldn't read file {:?}", path);
            HttpResponse::err()
        }
    
    }
}

fn read_template(path: &str) -> HttpResponse {
    debug!("read_template({path})");
    match fs::File::open(format!("template/{path}")) {
        Ok(mut f) => {
            let mut buffer = vec![];
            debug!("read {path} file");
            _=f.read_to_end(&mut buffer);
            debug!("file {path} read");
            HttpResponse::ok_from(buffer)
        },
        Err(_) => {
            error!("couldn't read template {:?}", path);
            HttpResponse::err()
        }
    
    }
} 
