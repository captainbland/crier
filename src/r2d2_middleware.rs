use std::any::Any;
use std::borrow::Cow;
use std::env;
use std::sync::Arc;

use diesel::pg::PgConnection;
use diesel::r2d2::*;
use dotenv::*;
use iron::{IronResult, middleware::{
        BeforeMiddleware,
    }, Request,
       Response,
       typemap::Key
};
use plugin::*;
use r2d2::Pool;
use core::borrow::Borrow;

pub struct R2D2Middleware {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>
}

impl R2D2Middleware {
    pub fn new() -> R2D2Middleware {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        println!("Database URL: {}", database_url);
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = Pool::builder().max_size(15).build(manager).expect("Failed to create pool - is your database available?.");
        return R2D2Middleware{pool: Arc::new(pool)};
    }
}

pub struct DatabaseExtension;

impl Key for DatabaseExtension {
    type Value = Arc<Pool<ConnectionManager<PgConnection>>>;
}

impl BeforeMiddleware for R2D2Middleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions_mut().entry::<DatabaseExtension>().or_insert(Arc::clone(&self.pool));
        Ok(())
    }
}
