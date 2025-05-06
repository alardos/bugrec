pub mod way_of_payment_repo;


#[derive(sqlx::FromRow, Debug)]
pub struct WayOfPayment {
    pub id: i64,
    pub name: String,
}