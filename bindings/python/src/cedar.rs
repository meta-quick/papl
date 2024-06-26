use papl::CedarEngine;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;
use papl::*;
use serde::{Deserialize, Serialize};
use cedar_policy::*;
use regorus::Value;

#[pyclass(unsendable)]
pub struct PaplCedarEngine {
    engine: CedarEngine
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum PyDecision {
    Allow,
    Deny,
}

impl From<Decision> for PyDecision {
    fn from(decision: Decision) -> Self {
        match decision {
            Decision::Allow => PyDecision::Allow,
            Decision::Deny => PyDecision::Deny,
        }
    }
}


#[pymethods]
impl PaplCedarEngine {
    #[new]
    pub fn new() -> Self {
        PaplCedarEngine {
            engine: CedarEngine::new()
        }
    }

    pub fn add_policy(&mut self, policy: String) -> PyResult<()> {
        let ret = self.engine.add_policy(policy);
        match ret {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn add_entity(&mut self, json: &str,) -> PyResult<()> {
        let ret = self.engine.add_entity(json);
        match ret {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn decide_request(&self, principal: String, action: String, resource: String, json_context: String,py : Python<'_>) -> PyResult<PyObject> {
        let ret = self.engine.decide_request(principal, action, resource, json_context);
        let py_decision = to(ret, py);
        match py_decision {
            Ok(py_decision) => Ok(py_decision),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }
}

fn to(mut v: Decision, py: Python<'_>) -> anyhow::Result<PyObject> {
    match v {
        Decision::Allow => Ok("ALLOW".to_object(py)),
        Decision::Deny => Ok("DENY".to_object(py)),
    }
}