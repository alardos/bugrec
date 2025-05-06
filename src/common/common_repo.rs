use std::sync::Mutex;

use sqlx::{Error, Pool, Postgres, PgConnection, ConnectOptions};
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};

pub async fn get_pool() -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:1234@localhost/postgres").await
}

pub static DB_POOL: Option<Mutex<Pool<Postgres>>> = Option::None;

pub struct Location {
    pub longitude: f64,
    pub lantitude: f64,
}

impl Location {
    
    pub fn avg_loc(&self, b: &Self) -> Location {
        Location {
            longitude: (self.longitude + b.longitude) / 2.0,
            lantitude: (self.lantitude + b.lantitude) / 2.0
        }
    }

    pub fn metric_distance(&self, b: &Self) -> f64 {
        ((self.lantitude - b.lantitude).powi(2) + (self.lantitude - b.longitude).powi(2))/ 2.0
    }

//    pub fn avg_loc_vec(locations: &Vec<Location>) -> Location {
//        *locations.iter().reduce(|acc, n| &acc.avg_loc(&n)).unwrap()
//    }
}
