use http::json::{Json, Json::Str, JsonArr, ToJson, JsonObj};



#[derive(sqlx::FromRow, Debug)]
pub struct Partner {
    pub id: i64,
    pub name: String,
    pub special: bool, 
}    

#[derive(sqlx::FromRow, Debug)]
pub struct PartnerWithTags {
    pub id: i64,
    pub name: String,
    pub special: bool,
    pub tags: Vec<String>,
}

impl ToString for Partner {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}


impl Clone for Partner {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), name: self.name.clone(), special: self.special.clone() }
    }
}

impl Partner {
    pub fn new(name: &str) -> Self {
        Self {
            name:name.to_string(),
            id: -1,
            special: false,
        }
    }

}

impl ToJson for Partner {
    fn to_json(&self) -> Json {
        let mut json = JsonObj::new();
        json.push("id", Str(self.id.to_string()));
        json.push("name", Str(self.name.to_string()));
        json.push("special", Str(self.special.to_string()));
        json.into()
    }
}

impl PartnerWithTags {
    pub fn new(name: &str) -> Self {
        Self {
            name:name.to_string(),
            id: -1,
            special: false,
            tags: vec![]
        }
    }

    pub fn to_json(&self) -> JsonObj {
        let mut json = JsonObj::new();
        json.push("id", Str(self.id.to_string()));
        json.push("name", Str(self.name.to_string()));
        json.push("special", Str(self.special.to_string()));
        json.push("tags", Arr(self.tags.iter().map(|t|Str(t.to_string())).collect()));
        json
    }
}
