use diesel::r2d2::*;
use r2d2::Pool;
use diesel::pg::PgConnection;
use dotenv::*;
use std::env;
use std::any::Any;
use std::sync::Arc;
use std::borrow::Cow;
use plugin::*;
use iron::{Request, Response, IronResult,
    middleware::{
        BeforeMiddleware,
    },
    typemap::Key,
    request::*
};


pub struct R2D2Middleware {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>
}

impl R2D2Middleware {
    pub fn new() -> R2D2Middleware {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().max_size(15).build(manager).expect("Failed to create pool - is your database available?.");
        return R2D2Middleware{pool: Arc::new(pool.clone())};
    }
}

pub struct DatabaseExtension;

impl Key for DatabaseExtension {
    type Value = Arc<Pool<ConnectionManager<PgConnection>>>;
}

impl BeforeMiddleware for R2D2Middleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions_mut().entry::<DatabaseExtension>().or_insert(Arc::from(self.pool.clone()));
        Ok(())
    }
}