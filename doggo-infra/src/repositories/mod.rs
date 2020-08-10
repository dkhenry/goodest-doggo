use std::sync::Arc;
use std::sync::Mutex;

// Convenience type for handlers.
#[derive(Clone)]
pub struct ConfigurablePool {
    inner: Option<mysql::Pool>
}

#[derive(Clone)]
pub struct Pool(Arc<Mutex<ConfigurablePool>>);

impl Pool {
    fn new(inner: Option<mysql::Pool>) -> Self {
        Self{
            0: Arc::new(Mutex::new(ConfigurablePool{
                inner: inner
            }))
        }
    }

    pub fn get_conn(&self) -> Result<mysql::PooledConn, mysql::Error> {
        let guard = &self.0.lock().unwrap();
        let inner = guard.inner.as_ref().unwrap();
        inner.get_conn()
    }

    pub fn set_url(&self, url: impl AsRef<str>) {
        let guard = &mut self.0.lock().unwrap();
        guard.inner = Some(mysql::Pool::new(&url).unwrap());
    }

    pub fn is_configured(&self) -> bool {
        let guard = &self.0.lock().unwrap();
        guard.inner.is_some()
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
