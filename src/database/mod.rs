use salvo::prelude::*;

use std::sync::Arc;
use tokio::sync::Mutex;


pub use sqlx;

pub type ConnOptions = sqlx::any::AnyPoolOptions;
pub type ConnPool = sqlx::AnyPool;
pub type Connection = sqlx::AnyConnection;
pub type DatabaseHandler<T> = Arc<Mutex<Database<T>>>;


pub struct Database<T>(Vec<(T, ConnPool)>)
where 
    T: PartialEq + Send + Sync + 'static;

impl<T: PartialEq + Send + Sync + 'static> Database<T> {

    pub fn new() -> DatabaseHandler<T> {
        sqlx::any::install_default_drivers();
        Arc::new(Mutex::new(Self(Vec::new())))
    }

    pub fn exists(&mut self, key: T) -> bool {
        self.0.iter().find(|item| item.0 == key).is_some()
    }

    pub fn add(&mut self, key: T, pool: ConnPool) {
        self.0.push((key, pool));
    }

    pub fn get(&self, key: T) -> Option<&(T, ConnPool)> {
        self.0.iter().find(|item| item.0 == key)
    }

}



pub trait DatabaseDepotExt<T>
where 
    T: PartialEq + Send + Sync + 'static
{
    fn database(&self, key: T) -> impl std::future::Future<Output = Result<ConnPool, DatabaseError>> + Send;
}

impl<T: PartialEq + Send + Sync + 'static> DatabaseDepotExt<T> for Depot {

    fn database(&self, key: T) -> impl std::future::Future<Output = Result<ConnPool, DatabaseError>> + Send {
        async {
            let guard = self.get_typed::<DatabaseHandler<T>>().unwrap().lock().await;
            let conn = guard.get(key).map(|(_, c)| c.clone()).ok_or(DatabaseError::DatabasePoolNotFound)?;
            drop(guard);
            Ok(conn)
        }
    }

}



#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error("database pool not found.")]
    DatabasePoolNotFound,
}