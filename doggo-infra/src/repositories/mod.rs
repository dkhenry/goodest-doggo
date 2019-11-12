lazy_static! {
    static ref CLIENT_POOL: mysql::Pool = {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        mysql::Pool::new(&database_url).unwrap()
    };
}

// Convenience type for handlers.
type Conn = mysql::PooledConn;

pub mod pupper_repository;
pub use pupper_repository::*;
pub mod ballot_repository;
pub use ballot_repository::*;
pub mod user_repository;
pub use user_repository::*;
