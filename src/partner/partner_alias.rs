
#[derive(sqlx::FromRow, Debug)]
pub struct PartnerAlias { 
    pub id: i64, 
    pub name: String, 
    pub partner_id: i64 
}