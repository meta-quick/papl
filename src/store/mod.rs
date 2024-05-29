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

use rusqlite::{Connection,Result};

pub struct SqliteStore {
    conn: Connection,
}

fn table_exists(conn: &Connection, table_name: &str) -> bool {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?").unwrap();
    let mut rows = stmt.query(&[&table_name]);

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
                    key TEXT NOT NULL,
                    version TEXT NOT NULL,
                    policy TEXT NOT NULL
                )",
                (), // empty list of parameters.
            )?;
        }

        Ok(SqliteStore { conn })
    }

    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        //check if table exists, if not create it
        if !table_exists(&conn, "policy") {
            conn.execute(
                "CREATE TABLE policy (
                    id INTEGER PRIMARY KEY,
                    key TEXT NOT NULL,
                    version TEXT NOT NULL,
                    policy TEXT NOT NULL
                )",
                (), // empty list of parameters.
            )?;
        }

        Ok(SqliteStore { conn })
    }

    pub fn save(&self, key: String, policy: String, version: String) -> Result<usize> {
        let conn = &self.conn;
        //update if key exists
        let mut stmt = conn.prepare("UPDATE policy SET policy = ?1, version = ?2 WHERE key = ?3")?;
        let result = stmt.execute(&[&policy,&version, &key]);

        if let Ok(result) = result {
            if result > 0 {
                return Ok(result);
            }
        }

        //insert if key does not exist
        let mut stmt = conn.prepare("INSERT INTO policy (key,version, policy) VALUES (?1, ?2, ?3)")?;
        let result = stmt.execute(&[&key, &version, &policy]);
        result
    }

    pub fn get(&self, key: String) -> Result<String> {
        let conn = &self.conn;
        let mut stmt = conn.prepare("SELECT policy FROM policy WHERE key = ?")?;
        let mut rows = stmt.query(&[&key]);


        if let Ok(mut rows) = rows {
            if let Ok(Some(row)) = rows.next() {
                let policy: String = row.get(0)?;
                return Ok(policy);
            }
        }
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    pub fn version(&self, key: String) -> Result<String> {
        let conn = &self.conn;
        let mut stmt = conn.prepare("SELECT version FROM policy WHERE key = ?")?;
        let mut rows = stmt.query(&[&key]);


        if let Ok(mut rows) = rows {
            if let Ok(Some(row)) = rows.next() {
                let version: String = row.get(0)?;
                return Ok(version);
            }
        }
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    pub fn version_value(&self, key: String) -> Result<(String,String)> {
        let conn = &self.conn;
        let mut stmt = conn.prepare("SELECT policy,version FROM policy WHERE key = ?")?;
        let mut rows = stmt.query(&[&key]);


        if let Ok(mut rows) = rows {
            if let Ok(Some(row)) = rows.next() {
                let policy: String = row.get(0)?;
                let version: String = row.get(1)?;
                return Ok((policy,version));
            }
        }
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    pub fn delete(&self, key: String) -> Result<usize> {
        let conn = &self.conn;
        let mut stmt = conn.prepare("DELETE FROM policy WHERE key = ?")?;
        let result = stmt.execute(&[&key]);
        result
    }

    pub fn close(&self) -> Result<()> {
        let conn = &self.conn;
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use std::ops::Add;
    use super::*;
    #[test]
    fn test_sqlite_store() -> Result<()> {
        let store = SqliteStore::new("demo")?;
        store.save("key1".to_string(), "policy10".to_string())?;
        store.save("key2".to_string(), "policy20".to_string())?;
        store.save("key3".to_string(), "policy30".to_string())?;

        assert_eq!(store.get("key1".to_string())?, "policy10".to_string());
        assert_eq!(store.get("key2".to_string())?, "policy20".to_string());

        Ok(())
    }

    #[test]
    fn test_sqlite_store_delete() -> Result<()> {
        let store = SqliteStore::new("demo")?;
        store.delete("key1".to_string());
        Ok(())
    }
}