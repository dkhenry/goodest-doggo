use std::sync::Arc;
use std::sync::Mutex;

use mysql::prelude::FromRow;
use mysql::prelude::Queryable;

// Convenience type for handlers.
#[derive(Clone)]
pub struct ConfigurablePool {
    inner: Option<mysql::Pool>,
    is_working: bool
}

#[derive(Clone)]
pub struct Pool(Arc<Mutex<ConfigurablePool>>);

// TODO:  impl Queryable for Pool
// TODO:  DRY
impl Pool {
    fn new(inner: Option<mysql::Pool>) -> Self {
        Self{
            0: Arc::new(Mutex::new(ConfigurablePool{
                inner: inner,
                is_working: true
            }))
        }
    }

    pub fn get_conn(&self) -> Result<mysql::PooledConn, mysql::Error> {
        let guard = &mut self.0.lock().unwrap();
        let inner = guard.inner.as_ref().unwrap();
        match inner.get_conn() {
            Ok(v) => Ok(v),
            Err(e) => {
                guard.is_working = false;
                Err(e)
            }
        }
    }

    pub fn set_url(&self, url: impl AsRef<str>) -> Result<(), mysql::Error> {
        let guard = &mut self.0.lock().unwrap();
        guard.inner = Some(match mysql::Pool::new(&url) {
            Ok(v) => {
                guard.is_working = true;
                v
            },
            Err(e) => {
                guard.is_working = false;
                return Err(e)
            }
        });
        Ok(())
    }

    pub fn is_configured(&self) -> bool {
        let guard = &self.0.lock().unwrap();
        guard.inner.is_some()
    }

    pub fn is_working(&self) -> bool {
        let guard = &self.0.lock().unwrap();
        guard.inner.is_some() && guard.is_working
    }

    pub fn query<T: FromRow>(&self, query: impl AsRef<str>) -> mysql::Result<Vec<T>> {
        let mut connection = self.get_conn()?;
        match connection.query(query.as_ref()) {
            Ok(v) => {
                self.0.lock().unwrap().is_working = true;
                Ok(v)
            },
            Err(e) => {
                self.0.lock().unwrap().is_working = false;
                Err(e)
            }
        }
    }

    pub fn query_first<T: FromRow>(&self, query: impl AsRef<str>) -> mysql::Result<Option<T>> {
        let mut connection = self.get_conn()?;
        match connection.query_first(query.as_ref()) {
            Ok(v) => {
                self.0.lock().unwrap().is_working = true;
                Ok(v)
            },
            Err(e) => {
                self.0.lock().unwrap().is_working = false;
                Err(e)
            }
        }
    }

    pub fn query_drop(&self, query: impl AsRef<str>) -> mysql::Result<()> {
        let mut connection = self.get_conn()?;
        match connection.query_drop(query.as_ref()) {
            Ok(v) => {
                self.0.lock().unwrap().is_working = true;
                Ok(v)
            },
            Err(e) => {
                self.0.lock().unwrap().is_working = false;
                Err(e)
            }
        }
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
