// Copyright 2024 brian <gao.brian@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::{Arc, Mutex};
use rusqlite::{Connection, Error, Result};

pub struct SqliteStore {
    conn: Arc<Mutex<Option<Connection>>>,
}

fn table_exists(conn: &Connection, table_name: &str) -> bool {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?").unwrap();
    let rows = stmt.query(&[&table_name]);

    if let Ok(mut rows) = rows {
        if let Ok(Some(row)) = rows.next() {
            let _policy: String = row.get(0).unwrap();
            return true
        }
    }

    false
}

impl SqliteStore {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        //check if table exists, if not create it
        if !table_exists(&conn, "policy") {
            println!("Table does not exist, creating it...");
            conn.execute(
                "CREATE TABLE policy (
                    id INTEGER PRIMARY KEY,
                    stamp LONG NOT NULL,
                    key TEXT NOT NULL,
                    version TEXT NOT NULL,
                    policy TEXT NOT NULL
                )",
                (), // empty list of parameters.
            )?;
            conn.execute(
                "CREATE UNIQUE INDEX policy_key_IDX ON policy (\"key\");",
                (), // empty list of parameters.
            )?;
        }

        Ok(SqliteStore { conn: Arc::new(Mutex::new(Some(conn))) })
    }

    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        //check if table exists, if not create it
        if !table_exists(&conn, "policy") {
            conn.execute(
                "CREATE TABLE policy (
                    id INTEGER PRIMARY KEY,
                    stamp LONG NOT NULL,
                    key TEXT NOT NULL,
                    version TEXT NOT NULL,
                    policy TEXT NOT NULL
                )",
                (), // empty list of parameters.
            )?;
            //Create indexes for key
            conn.execute(
                "CREATE UNIQUE INDEX policy_key_IDX ON policy (\"key\");",
                (), // empty list of parameters.
            )?;
        }

        Ok(SqliteStore { conn: Arc::new(Mutex::new(Some(conn))) })
    }

    pub fn save(&self, key: String, policy: String, version: String, timestamp: i64) -> Result<usize> {
        let lock_result = &self.conn.lock();
        match lock_result {
            Ok(conn_lock) => {
                //update if key exists
                let stamp = timestamp.to_string();
                let conn = conn_lock.as_ref().unwrap();
                let mut stmt = conn.prepare("UPDATE policy SET policy = ?1, version = ?2, stamp = ?4  WHERE key = ?3")?;
                let result = stmt.execute(&[&policy,&version, &key,&stamp]);

                match result {
                    Ok(result) => {
                        if result > 0 {
                            return Ok(result);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error updating policy: key: {}, {}",key, e);
                    }
                }

                //insert if key does not exist
                let mut stmt = conn.prepare("INSERT INTO policy (key,policy,version,stamp) VALUES (?1, ?2, ?3,?4)")?;
                let result = stmt.execute(&[&key, &policy, &version,&stamp]);

                match result {
                    Ok(result) => {
                        if result > 0 {
                            return Ok(result);
                        } else {
                            eprintln!("Error inserting policy: key: {}", key);
                            return Err(rusqlite::Error::QueryReturnedNoRows);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error inserting policy: key: {}, {}", key,e);
                        Err(e)
                    }
                }
            }
            Err(_) => {
                Err(Error::SqliteSingleThreadedMode)
            }
        }
    }

    pub fn get(&self, key: String) -> Result<String> {
        let lock_result = &self.conn.lock();
        match lock_result {
            Ok(conn_lock) => {
                let conn = conn_lock.as_ref().unwrap();
                let mut stmt = conn.prepare("SELECT policy FROM policy WHERE key = ?")?;
                let rows = stmt.query(&[&key]);


                if let Ok(mut rows) = rows {
                    if let Ok(Some(row)) = rows.next() {
                        let policy: String = row.get(0)?;
                        return Ok(policy);
                    }
                }
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }
            Err(_) => {
                Err(Error::SqliteSingleThreadedMode)
            }
        }
    }

    pub fn version(&self, key: String) -> Result<String> {
        let lock_result = &self.conn.lock();
        match lock_result {
            Ok(conn) => {
                let conn = conn.as_ref().unwrap();
                let mut stmt = conn.prepare("SELECT version FROM policy WHERE key = ?")?;
                let rows = stmt.query(&[&key]);


                if let Ok(mut rows) = rows {
                    if let Ok(Some(row)) = rows.next() {
                        let version: String = row.get(0)?;
                        return Ok(version);
                    }
                }
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }
            Err(_) => {
                Err(Error::SqliteSingleThreadedMode)
            }
        }
    }

    pub fn version_value(&self, key: String) -> Result<(String,String)> {
        let lock_result = &self.conn.lock();
        match lock_result {
            Ok(conn) => {
                let conn = conn.as_ref().unwrap();
                let mut stmt = conn.prepare("SELECT policy,version FROM policy WHERE key = ?")?;
                let  rows = stmt.query(&[&key]);


                if let Ok(mut rows) = rows {
                    if let Ok(Some(row)) = rows.next() {
                        let policy: String = row.get(0)?;
                        let version: String = row.get(1)?;
                        return Ok((policy,version));
                    }
                }
                Err(Error::QueryReturnedNoRows)
            }
            Err(_) => {
                Err(Error::SqliteSingleThreadedMode)
            }
        }
    }

    pub fn all_keys_le(&self, timestamp: i64) -> Result<Vec<String>> {
        let lock_result = &self.conn.lock();
        match lock_result {
            Ok(conn) => {
                let conn = conn.as_ref().unwrap();
                let stamp = timestamp.to_string();
                let mut stmt = conn.prepare("SELECT key FROM policy WHERE stamp <= ?1")?;
                let mut rows = stmt.query(&[&stamp])?;

                let mut keys = Vec::new();
                while let Ok(Some(row)) = rows.next() {
                    let key: String = row.get(0)?;
                    keys.push(key);
                }

                Ok(keys)
            }
            Err(_) => {
                Err(Error::SqliteSingleThreadedMode)
            }
        }
    }

    pub fn all_keys_be(&self, timestamp: i64) -> Result<Vec<String>> {
        let lock_result = &self.conn.lock();
        match lock_result {
            Ok(conn) => {
                let conn = conn.as_ref().unwrap();
                let stamp = timestamp.to_string();
                let mut stmt = conn.prepare("SELECT key FROM policy WHERE stamp >= ?1")?;
                let mut rows = stmt.query(&[&stamp])?;

                let mut keys = Vec::new();
                while let Ok(Some(row)) = rows.next() {
                    let key: String = row.get(0)?;
                    keys.push(key);
                }

                Ok(keys)
            }
            Err(_) => {
                Err(Error::SqliteSingleThreadedMode)
            }
        }
    }

    pub fn all_keys_be_pageable(&self,timestamp: i64, page: i64, size: i64) -> Result<Vec<String>> {
        let lock_result = &self.conn.lock();
        match lock_result {
            Ok(conn) => {
                let conn = conn.as_ref().unwrap();
                let stamp = timestamp.to_string();
                let offset = (page - 1) * size;
                let offset = offset.to_string();
                if size == 0 {
                    return Ok(Vec::new());
                }

                let size = size.to_string();
                let mut stmt = conn.prepare("SELECT key FROM policy WHERE stamp >= ?1 LIMIT ?2 OFFSET ?3")?;
                let mut rows = stmt.query(&[&stamp,&size,&offset])?;

                let mut keys = Vec::new();
                while let Ok(Some(row)) = rows.next() {
                    let key: String = row.get(0)?;
                    keys.push(key);
                }

                Ok(keys)
            }
            Err(_) => {
                Err(Error::SqliteSingleThreadedMode)
            }
        }
    }

    pub fn evict_be(&self, timestamp: i64) -> Result<usize> {
        let lock_result = &self.conn.lock();
        match lock_result {
            Ok(conn) => {
                let conn = conn.as_ref().unwrap();
                if table_exists(&conn, "policy") {
                    let stamp = timestamp.to_string();
                    let mut stmt = conn.prepare("DELETE FROM policy WHERE stamp >= ?1")?;
                    let result = stmt.execute(&[&stamp]);
                    result
                } else {
                    Ok(0)
                }
            }
            Err(_) => {
                return Err(Error::SqliteSingleThreadedMode)
            }
        }
    }

    pub fn evict_le(&self, timestamp: i64) -> Result<usize> {
        let lock_result = &self.conn.lock();
        match lock_result {
            Ok(conn) => {
                let conn = conn.as_ref().unwrap();
                if table_exists(&conn, "policy") {
                    let stamp = timestamp.to_string();
                    let mut stmt = conn.prepare("DELETE FROM policy WHERE stamp <= ?1")?;
                    let result = stmt.execute(&[&stamp]);
                    result
                } else {
                    Ok(0)
                }
            }
            Err(_) => {
                return Err(Error::SqliteSingleThreadedMode)
            }
        }
    }

    pub fn delete(&self, key: String) -> Result<usize> {
        let lock_result = &self.conn.lock();
        match lock_result {
            Ok(conn) => {
                let conn = conn.as_ref().unwrap();
                let mut stmt = conn.prepare("DELETE FROM policy WHERE key = ?")?;
                let result = stmt.execute(&[&key]);
                result
            }
            Err(_) => {
                return Err(Error::SqliteSingleThreadedMode)
            }
        }
    }

    pub fn close(&self) -> Result<()> {
        let mut conn_guard = self.conn.lock().map_err(|_| Error::SqliteSingleThreadedMode)?;

        // Take the connection out of the MutexGuard, replacing it with None
        if let Some(conn) = conn_guard.take() {
            // Now you can close the connection because you own it
            conn.close().map_err(|_| Error::SqliteSingleThreadedMode)?;
        }

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sqlite_store() -> Result<()> {
        let store = SqliteStore::new("demo.db")?;
        store.save("key1".to_string(), "policy10".to_string(), "1.0".to_string(), 1)?;
        store.save("key2".to_string(), "policy20".to_string(), "2.0".to_string(),2)?;
        store.save("key3".to_string(), "policy30".to_string(), "3.0".to_string(),3)?;
        store.save("key3".to_string(), "policy30".to_string(), "3.0".to_string(),3)?;

        assert_eq!(store.get("key1".to_string())?, "policy10".to_string());
        assert_eq!(store.get("key2".to_string())?, "policy20".to_string());

        let keys = store.all_keys_be(2);
        for key in keys.unwrap() {
            println!("key: {}", key);
        }

        let keys = store.all_keys_le(2);
        for key in keys.unwrap() {
            println!("key: {}", key);
        }

        Ok(())
    }

    #[test]
    fn test_sqlite_pageable() -> Result<()> {
        let store = SqliteStore::new("demo.db")?;
        store.save("key1".to_string(), "policy10".to_string(), "1.0".to_string(), 1)?;
        store.save("key2".to_string(), "policy20".to_string(), "2.0".to_string(),2)?;
        store.save("key3".to_string(), "policy30".to_string(), "3.0".to_string(),3)?;

        assert_eq!(store.get("key1".to_string())?, "policy10".to_string());
        assert_eq!(store.get("key2".to_string())?, "policy20".to_string());

        let keys = store.all_keys_be_pageable(1, 2, 2);
        for key in keys.unwrap() {
            println!("key: {}", key);
        }


        Ok(())
    }

    #[test]
    fn test_sqlite_store_delete() -> Result<()> {
        let store = SqliteStore::new("demo.db")?;
        let _ = store.delete("key1".to_string());
        Ok(())
    }

    #[test]
    fn test_sqlite_store_close() -> Result<()> {
        let store = SqliteStore::new("demo.db")?;
        store.close().unwrap();
        Ok(())
    }
}