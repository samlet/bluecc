extern crate diesel_migrations;

use self::diesel_migrations::run_pending_migrations;
use diesel::prelude::*;
use dotenv;
use std::sync::{Mutex, MutexGuard};

pub fn test_connection() -> PgConnection {
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&url).unwrap();
    run_pending_migrations(&conn).unwrap();
    conn.begin_test_transaction().unwrap();
    conn
}

pub fn get_connection() -> PgConnection {
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&url).unwrap();
    conn
}

pub fn this_test_modifies_env() -> MutexGuard<'static, ()> {
    let _ = dotenv::var("FORCING_DOTENV_LOAD");
    lazy_static! {
        static ref ENV_LOCK: Mutex<()> = Mutex::new(());
    }
    ENV_LOCK.lock().unwrap()
}

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, Error};
use std::env;

type PgPool = Pool<ConnectionManager<PgConnection>>;
type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

fn pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

pub fn establish_connection_with_pool() -> PooledPg {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env not set");
    pg_pool(database_url.as_str()).get().unwrap()
}
