pub mod purchase_controller;
pub mod purchase_repo;

use http::json::{ToJson, Json, JsonObj};
use serde::Deserialize;
use http::json::Json::Str;


#[derive(sqlx::FromRow, Debug, Deserialize)]
pub struct Purchase {
    pub id: Option<i64>,
    pub item_id: i64,
    pub time: i64,
    pub partner_id: i64,
    pub sum: f64,
    pub transfer_id: i64,
}


impl ToJson for Purchase {
    fn to_json(&self) -> Json {
        let mut json = JsonObj::new();

        if let Some(id) = self.id { json.push("id", Str(id.to_string())); }

        json.push("item_id", Str(self.item_id.to_string()));
        json.push("time", Str(self.time.to_string()));
        json.push("partner_id", Str(self.partner_id.to_string()));
        json.push("sum", Str(self.sum.to_string()));
        json.push("transfer_id", Str(self.transfer_id.to_string()));

        json.into()
    }
}

impl ToString for Purchase {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}
