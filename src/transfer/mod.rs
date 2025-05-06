use http::json::{ToJson, Json, Json::Str, JsonObj};


pub mod transfer_repo;
pub mod transfer_controller;


#[derive(sqlx::FromRow, Debug)]
pub struct Transfer {
    pub id: i64,
    pub auto_category: String,
    pub currency: String,
    pub description: String,
    pub sum: f64,
    pub original_balance: f64,
    pub time: i64,
    pub partner_id: i64,
    pub way_of_payment_id: i64,
    pub assigned: bool,
    pub note: String
}

impl ToJson for Transfer {
    fn to_json(&self) -> Json {
        let mut json = JsonObj::new();
        json.push("id", Str(self.id.to_string()));
        json.push("auto_category", Str(self.auto_category.to_string()));
        json.push("currency", Str(self.currency.to_string()));
        json.push("description", Str(self.description.to_string()));
        json.push("sum", Str(self.sum.to_string()));
        json.push("original_balance", Str(self.original_balance.to_string()));
        json.push("time", Str(self.time.to_string()));
        json.push("partner_id", Str(self.partner_id.to_string()));
        json.push("way_of_payment", Str(self.way_of_payment_id.to_string()));
        json.push("assigned", Str(self.assigned.to_string()));
        json.push("note", Str(self.note.to_string()));
        json.into()
    }
}

impl ToString for Transfer {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}

