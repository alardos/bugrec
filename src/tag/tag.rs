use http::json::{Json, ToJson, JsonObj};
use serde::{Deserialize, Serialize};


#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
    pub parent_id: i64,
}

impl From<Vec<Tag>> for Json {
    fn from(val: Vec<Tag>) -> Self {
        val.iter().map(Tag::to_json).collect::<Json>()
    }
}

impl From<&Vec<Tag>> for Json{
    fn from(val: &Vec<Tag>) -> Self {
        val.iter().map(Tag::to_json).collect::<Json>()
    }
}

impl From<Tag> for Json {
    fn from(val: Tag) -> Self { val.to_json() }
}

impl From<&Tag> for Json {
    fn from(val: &Tag) -> Self { val.to_json() }
}

impl ToJson for Tag {
    fn to_json(&self) -> Json {
        let mut json = JsonObj::new();

        if let Some(id) = self.id { json.push("id", Json::Str(id.to_string())); }
        json.push("name", Json::Str(self.name.to_string()));
        json.push("parent_id", Json::Str(self.parent_id.to_string()));
        
        json.into()
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}

impl Clone for Tag {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), name: self.name.clone(), parent_id: self.parent_id.clone() }
    }
}
