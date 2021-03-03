#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
mod errors;
mod api;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, Error};
use crate::errors::AppError;

pub type AppResult<T> = Result<T, AppError>;
// pub type AppResult<T> = Result<T, Box<dyn AppError>>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

type PgPool = Pool<ConnectionManager<PgConnection>>;
type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

fn pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

fn establish_connection_with_pool() -> PooledPg {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env not set");
    pg_pool(database_url.as_str()).get().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
