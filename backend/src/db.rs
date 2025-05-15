use core::fmt;
use std::path::PathBuf;

use async_dup::Arc;
use async_lock::RwLock;
use surrealkv::{Options, Store};

use crate::{BackendError, BackendResult, KV, PROJECTS_DB, PUBLISHERS_DB};

pub struct DbState {
    publishers: Arc<RwLock<Store>>,
    projects: Arc<RwLock<Store>>,
}

impl DbState {
    pub async fn init() -> BackendResult<()> {
        let new_self = Self {
            publishers: Arc::new(RwLock::new(Self::store_ops(PUBLISHERS_DB)?)),
            projects: Arc::new(RwLock::new(Self::store_ops(PROJECTS_DB)?)),
        };

        KV.set(new_self)
            .await
            .or(Err(BackendError::KvAlreadyInitialized))?;

        Ok(())
    }

    fn store_ops(db_name: &str) -> BackendResult<Store> {
        let mut path = PathBuf::new();
        path.push(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push("DATABASE");
        path.push(db_name);

        let mut opts = Options::new();
        opts.dir = path;

        Ok(Store::new(opts)?)
    }

    async fn set_key_value(db_name: &str, key: &str, value: &[u8]) -> BackendResult<()> {
        let mut txn = Self::get_store()?.get_db(db_name).write().await.begin()?;
        let key = key.as_bytes();

        txn.set(key, value)?;
        txn.commit()?;

        Ok(())
    }

    pub async fn create(db_name: &str, key: &str, value: &[u8]) -> BackendResult<()> {
        if let Err(error) = Self::read(db_name, key).await {
            match error {
                BackendError::KvKeyNotFound => Self::set_key_value(db_name, key, value).await,
                _ => Err(error),
            }
        } else {
            Err(BackendError::KvAlreadyExists)
        }
    }

    pub async fn update(db_name: &str, key: &str, value: &[u8]) -> BackendResult<()> {
        Self::read(db_name, key).await?;
        Self::set_key_value(db_name, key, value).await
    }

    pub async fn purge(db_name: &str, key: &str) -> BackendResult<()> {
        let mut txn = Self::get_store()?.get_db(db_name).write().await.begin()?;
        let key = key.as_bytes();

        txn.delete(key)?;
        txn.commit()?;

        Ok(())
    }

    pub async fn remove(db_name: &str, key: &str) -> BackendResult<()> {
        Self::read(db_name, key).await?;

        let mut txn = Self::get_store()?.get_db(db_name).write().await.begin()?;
        let key = key.as_bytes();

        txn.delete(key)?;
        txn.commit()?;

        Ok(())
    }

    pub async fn read(db_name: &str, key: &str) -> BackendResult<Vec<u8>> {
        Self::get_store()?
            .get_db(db_name)
            .read()
            .await
            .begin()?
            .get(key.as_bytes())?
            .ok_or(BackendError::KvKeyNotFound)
    }

    pub async fn values(db_name: &str) -> BackendResult<Vec<Vec<u8>>> {
        let results = Self::get_store()?
            .get_db(db_name)
            .read()
            .await
            .begin()?
            .scan(.., None)
            .map(|value| {
                if let Ok((_, two, _)) = value {
                    two
                } else {
                    Vec::default()
                }
            })
            .collect();

        Ok(results)
    }

    pub fn get_db(&self, db_name: &str) -> Arc<RwLock<Store>> {
        if db_name.as_bytes() == b"PUBLISHERS" {
            self.publishers.clone()
        } else {
            self.projects.clone()
        }
    }

    pub fn get_store() -> BackendResult<&'static DbState> {
        KV.get().ok_or(BackendError::KvUninitialized)
    }
}

impl fmt::Debug for DbState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbState").finish()
    }
}
