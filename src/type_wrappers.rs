use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;

// Generally wraps any third party dependencies which we might want to mock somehow

pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

//#[cfg(not(tests))]
pub type Session = iron_sessionstorage::Session;

