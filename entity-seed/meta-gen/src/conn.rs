use diesel::{PgConnection, Connection};

pub fn establish_connection() -> PgConnection {
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url="postgres://xiaofeiwu:@localhost:5432/seed";
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

