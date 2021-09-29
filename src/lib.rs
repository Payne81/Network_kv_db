use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    kv_db: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore{
        let mut kv_db: HashMap<String, String> = HashMap::new();
        KvStore { kv_db }
    }

    pub fn set(&mut self, key: String, value: String) {
        // println!("KvStore set {} , {}", key, value);
        self.kv_db.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        // println!("KvStore get {}", key);
        match self.kv_db.get(&key) {
            Some(val) => Some(val.clone()),
            _         => None,
        }
    }

    pub fn remove(&mut self, key: String){
        // let val = self.get(key);
        self.kv_db.remove(&key);
    }
}