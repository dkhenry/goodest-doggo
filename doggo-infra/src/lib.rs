#[macro_use]
extern crate lazy_static;

pub mod errors;
pub mod mysql_pool;
pub mod repositories;
pub use repositories::CLIENT_POOL;
