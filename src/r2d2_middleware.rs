use std::any::Any;
use std::borrow::Cow;
use std::env;
use std::sync::Arc;

use core::borrow::Borrow;
use diesel::pg::PgConnection;
use diesel::r2d2::*;
use dotenv::*;
use iron::{middleware::BeforeMiddleware, typemap::Key, IronResult, Request, Response};
use plugin::*;
use r2d2::Pool;

pub struct R2D2Middleware {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl R2D2Middleware {
    pub fn new() -> R2D2Middleware {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        info!("Database URL: {}", database_url);
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = Pool::builder()
            .min_idle(Some(15))
            .max_size(30)
            .test_on_check_out(true)
            .build(manager)
            .expect("Failed to create pool - is your database available?.");
        info!("Pool info: {:?}", pool.state());
        info!("Test on checkout: {:?}", pool.test_on_check_out());
        return R2D2Middleware {
            pool: Arc::new(pool),
        };
    }
}

pub struct DatabaseExtension;

impl Key for DatabaseExtension {
    type Value = Arc<Pool<ConnectionManager<PgConnection>>>;
}

impl BeforeMiddleware for R2D2Middleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions_mut()
            .entry::<DatabaseExtension>()
            .or_insert(Arc::clone(&self.pool));
        Ok(())
    }
}
