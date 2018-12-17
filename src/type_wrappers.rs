use r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;

// Generally wraps any third party dependencies which we might want to mock somehow

#[cfg(not(test))]
pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[cfg(not(test))]
pub type Session = iron_sessionstorage::Session;