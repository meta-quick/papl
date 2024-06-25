use papl::SqliteStore;

use std::collections::{BTreeMap, BTreeSet};
use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;
use papl::*;
use regorus::{Value};
use anyhow::{Result};

#[pyclass(unsendable)]
pub struct PaplStore {
    store: SqliteStore,
}

#[pymethods]
impl PaplStore {

    #[new]
    pub fn new(path: String, ty: String) -> Self {
        if ty == "memory" {
            let store = SqliteStore::new_in_memory();
            if store.is_err() {
                panic!("{}", store.err().expect("error occur while creating memory store").to_string());
            }
            return PaplStore { store: store.unwrap() };
        }

        let store = SqliteStore::new(&path);
        if store.is_err() {
            panic!("{}", store.err().expect("error occur while creating store").to_string());
        }
        PaplStore { store: store.unwrap() }
    }

    pub fn save(&mut self, key: String, policy: String, version: String) -> PyResult<usize> {
        let result = self.store.save(key, policy, version);
        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn get(&self, key: String) -> PyResult<String> {
        let result = self.store.get(key);
        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn version(&self, key: String) -> PyResult<String> {
        let result = self.store.version(key);
        match result {
            Ok(version) => {
                return Ok(version);
            },
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn delete(&self, key: String) -> PyResult<usize> {
        let result = self.store.delete(key);
        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn close(&self) -> PyResult<()>{
        let _ = self.store.close();
        Ok(())
    }

    pub fn value_with_version(&self, key: String) -> PyResult<(String, String)> {
        let result = self.store.version_value(key);
        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }
}

