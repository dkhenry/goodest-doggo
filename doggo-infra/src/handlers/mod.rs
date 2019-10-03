lazy_static! {
    static ref CLIENT_POOL: mysql::Pool = {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        mysql::Pool::new(&database_url).unwrap()
    };
}

// Convenience type for handlers.
type Conn = mysql::PooledConn;

pub mod query_handlers;
pub use query_handlers::*;

pub mod query_retry_wrapper;
pub use query_retry_wrapper::*;

pub mod command_handlers;
pub use command_handlers::*;
