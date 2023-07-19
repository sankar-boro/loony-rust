use std::sync::Mutex;
use std::collections::{HashMap, HashSet};

type Version = u64;
type Timestamp = u64;

pub struct Transaction {
    start_ts: Timestamp,
    read_set: HashSet<String>,
    write_set: HashMap<String, (Version, String)>,
}

pub struct MVCC {
    current_ts: Mutex<Timestamp>,
    committed_versions: Mutex<HashMap<String, (Version, String)>>,
}

impl MVCC {
    pub fn new() -> Self {
        MVCC {
            current_ts: Mutex::new(0),
            committed_versions: Mutex::new(HashMap::new()),
        }
    }

    pub fn begin_transaction(&self) -> Transaction {
        let start_ts = {
            let mut current_ts = self.current_ts.lock().unwrap();
            *current_ts += 1;
            *current_ts
        };
        Transaction {
            start_ts,
            read_set: HashSet::new(),
            write_set: HashMap::new(),
        }
    }

    pub fn read(&self, transaction: &mut Transaction, key: &str) -> Option<(Version, String)> {
        let committed_versions = self.committed_versions.lock().unwrap();
        if let Some((version, value)) = committed_versions.get(key) {
            if *version <= transaction.start_ts {
                transaction.read_set.insert(key.to_owned());
                Some((*version, String::from(value)))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn write(&self, transaction: &mut Transaction, key: String, value: String) {
        transaction.write_set.insert(key.clone(), (transaction.start_ts, value));
    }

    pub fn commit(&self, transaction: Transaction) -> bool {
        let mut committed_versions = self.committed_versions.lock().unwrap();
        for (key, (version, _)) in &transaction.write_set {
            if let Some(current_version) = committed_versions.get(key) {
                if current_version.0 > *version {
                    return false; // Abort transaction due to write-write conflict
                }
            }
        }
        for (key, (version, value)) in transaction.write_set {
            committed_versions.insert(key, (version, value));
        }
        true // Transaction committed successfully
    }
}