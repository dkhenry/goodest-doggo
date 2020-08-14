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
impl Pool {
    fn new(inner: Option<mysql::Pool>) -> Self {
        Self{
            0: Arc::new(Mutex::new(ConfigurablePool{
                is_working: inner.is_some(),
                inner: inner,
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
        let result = mysql::Pool::new(&url);
        guard.is_working = result.is_ok();
        guard.inner = Some(result?);
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
        let result = connection.query(query.as_ref());
        // TODO:  Consider specific errors to be a non-working database connection, rather than all of them
        self.0.lock().unwrap().is_working = result.is_ok();
        result
    }

    pub fn query_first<T: FromRow>(&self, query: impl AsRef<str>) -> mysql::Result<Option<T>> {
        let mut connection = self.get_conn()?;
        let result = connection.query_first(query.as_ref());
        // TODO:  Consider specific errors to be a non-working database connection, rather than all of them
        self.0.lock().unwrap().is_working = result.is_ok();
        result
    }

    pub fn query_drop(&self, query: impl AsRef<str>) -> mysql::Result<()> {
        let mut connection = self.get_conn()?;
        let result = connection.query_drop(query.as_ref());
        // TODO:  Consider specific errors to be a non-working database connection, rather than all of them
        self.0.lock().unwrap().is_working = result.is_ok();
        result
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
