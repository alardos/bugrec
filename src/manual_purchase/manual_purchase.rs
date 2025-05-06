#[derive(sqlx::FromRow, Debug)]
pub struct ManualPurchase {
    pub id: i64,
    pub time: i64,
    pub price: f64,
    pub longitude: f64,
    pub latitude: f64,
    pub partner_id: i64,
    pub item_id: i64
}