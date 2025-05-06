use http::json::{Json::Str, Json, JsonObj, ToJson};
use serde::Deserialize;

#[derive(sqlx::FromRow, Debug, Deserialize)]
pub struct Item {
    pub id: Option<i64>,
    pub bar_code: String,
    pub name: Option<String>,
    pub picture_url: Option<String>,
}

impl From<Vec<Item>> for Json {
    fn from(val: Vec<Item>) -> Self {
        val.iter().map(|p|p.to_json()).collect::<Json>()
    }
}

impl ToJson for Item {
    fn to_json(&self) -> Json {
        let mut json = JsonObj::new();

        json.push("bar_code", Str(self.bar_code.to_string()));
        if self.id.is_some() {
            json.push("id", Str(self.id.unwrap().to_string()));
        }
        if self.picture_url.is_some() {
            json.push("picture_url", Str(self.picture_url.clone().unwrap()));
        }
        if self.name.is_some() {
            json.push("name", Str(self.name.clone().unwrap()));
        }

        json.into()
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}
