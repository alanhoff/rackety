use std::sync::{RwLock, Arc};
use std::collections::HashMap;

pub type ArcStore = Arc<RwLock<HashMap<String, String>>>;

#[derive(Debug)]
pub enum Error {
    WriteError
}

pub fn new() -> ArcStore {
    Arc::new(RwLock::new(HashMap::new()))
}

pub fn safe_get(store: &ArcStore, key: &str) -> Option<String> {
    let store_read = match store.read() {
        Ok(read) => read,
        Err(_) => return None
    };

    match store_read.get(key) {
        Some(val) => Some(val.clone()),
        None => None
    }
}

pub fn safe_set(store: &ArcStore, key: String, value: String) -> Result<(), Error> {
    let mut store_write = match store.write() {
        Ok(write) => write,
        Err(_) => return Err(Error::WriteError)
    };

    store_write.insert(key, value);
    Ok(())
}

pub fn safe_remove(store: &ArcStore, key: &str) -> Result<(), Error> {
    let mut store_write = match store.write() {
        Ok(write) => write,
        Err(_) => return Err(Error::WriteError)
    };

    store_write.remove(key);
    Ok(())
}
