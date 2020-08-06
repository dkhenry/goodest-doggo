use std::sync::Arc;
use std::sync::Mutex;

// Convenience type for handlers.
#[derive(Clone)]
pub struct Pool(Arc<Mutex<Option<mysql::Pool>>>);

impl Pool {
    fn new(inner: Option<mysql::Pool>) -> Self {
        Self{
            0: Arc::new(Mutex::new(inner))
        }
    }

    pub fn get_conn(&self) -> Result<mysql::PooledConn, mysql::Error> {
        self.0.lock().unwrap().as_ref().unwrap().get_conn()
    }

    pub fn set_url(&self, url: impl AsRef<str>) {
        *self.0.lock().unwrap() = Some(mysql::Pool::new(&url).unwrap());
    }
}

lazy_static! {
    pub static ref CLIENT_POOL: Pool = {
        Pool::new(match std::env::var("DATABASE_URL") {
            Ok(url) => Some(mysql::Pool::new(&url).unwrap()),
            Err(_) => None
        })
    };
}

pub mod pupper_repository;
pub use pupper_repository::*;
pub mod ballot_repository;
pub use ballot_repository::*;
pub mod user_repository;
pub use user_repository::*;
