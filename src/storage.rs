use crate::project::File;
use rocksdb::DB;
use serde::Serialize;
use std::fmt::Debug;

pub fn put<T: Serialize + Debug>(db: &DB, key: String, file: T) -> &DB {
    match serde_json::to_string(&file) {
        Ok(o) => {
            match db.put(key, o) {
                Err(_e) => {}
                Ok(_ok) => {}
            };
        }
        Err(e) => {
            println!("{file:#?}");
            print!("{e}");
        }
    };
    db
}

pub fn get(db: &DB, key: String) -> Option<File> {
    match db.get(key) {
        Ok(Some(f)) => {
            let file = serde_json::from_slice(&f).unwrap();
            Some(file)
        }

        Err(e) => {
            println!("{e}");
            None
        }
        _ => None,
    }
}
