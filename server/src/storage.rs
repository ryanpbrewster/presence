use std::collections::BTreeMap;
use std::collections::Bound;
use std::sync::{Arc, Mutex};

use futures::sync::mpsc::UnboundedReceiver;

use crate::KvResult;

pub trait StorageLayer: Clone {
    fn put(&self, key: String, value: String) -> KvResult<()>;
    fn get(&self, key: &str) -> KvResult<Option<String>>;
    fn scan(&self, start: Bound<String>, end: Bound<String>)
        -> UnboundedReceiver<(String, String)>;
}

#[derive(Clone, Debug, Default)]
pub struct InMemoryStorageLayer {
    data: Arc<Mutex<BTreeMap<String, String>>>,
}

impl StorageLayer for InMemoryStorageLayer {
    fn put(&self, key: String, value: String) -> KvResult<()> {
        self.data.lock().unwrap().insert(key, value);
        Ok(())
    }

    fn get(&self, key: &str) -> KvResult<Option<String>> {
        Ok(self.data.lock().unwrap().get(key).cloned())
    }

    fn scan(
        &self,
        start: Bound<String>,
        end: Bound<String>,
    ) -> UnboundedReceiver<(String, String)> {
        let (sender, receiver) = futures::sync::mpsc::unbounded();
        let db = self.data.clone();
        println!("starting scan");
        ::std::thread::spawn(move || {
            for (k, v) in db.lock().unwrap().range((start, end)) {
                sender.unbounded_send((k.to_owned(), v.to_owned())).unwrap();
            }
            println!("done scanning");
        });
        receiver
    }
}
