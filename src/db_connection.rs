use lazy_static;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use std::sync::Arc;
use r2d2::PooledConnection;
use std::env;

lazy_static! {
    static ref CONNECTION_POOL: Arc<Pool<ConnectionManager<PgConnection>>> = {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        println!("Database URL: {}", database_url);
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = Pool::builder().max_size(15).build(manager).expect("Failed to create pool - is your database available?.");
        Arc::new(pool)
    };
}

pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    CONNECTION_POOL.get().expect("There must be a valid connection pool")
}