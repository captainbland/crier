use lazy_static;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use std::sync::Arc;
use r2d2::PooledConnection;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use std::env;
use std::thread;

lazy_static! {
    static ref CONNECTION_POOL: Pool<ConnectionManager<PgConnection>> = {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        info!("Database URL: {}", database_url);
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool - is your database available?.");

        for i in 0..10i32 {
            let pool = pool.clone();
            thread::spawn(move || {
                let conn = pool.get().unwrap();
            });
        }
        pool.clone()
    };
}

pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {

    info!("Getting connection!");
    let con = CONNECTION_POOL.get().expect("There must be a valid connection pool");
    info!("Got connection");
    con
}