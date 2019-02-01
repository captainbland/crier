use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;

use diesel::backend::Backend;
use diesel::connection::Connection;
use diesel::connection::TransactionManager;
use iron::IronResult;
use crate::user_model::UserSession;
// Generally wraps any third party dependencies which we might want to mock somehow

pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

//#[cfg(not(tests))]
pub type Session = iron_sessionstorage::Session;

//#[cfg(tests)]
//#[mock]
//pub trait TestSession<T> {
//    fn get(&self) -> IronResult<Option<T>>;
//    fn set(&mut self, t: T) -> IronResult<()>;
//    fn clear(&mut self) -> IronResult<()>;
//}
//
//#[cfg(tests)]
//pub type Session = MockTestSession<UserSession>;
