use std::collections::{BTreeMap, BTreeSet};
use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;
use papl::*;
use regorus::{Value};
use anyhow::{Result};


fn from(ob: &Bound<'_, PyAny>) -> Result<Value, PyErr> {
    // dicts
    Ok(if let Ok(dict) = ob.downcast::<PyDict>() {
        let mut map = BTreeMap::new();
        for (k, v) in dict {
            map.insert(from(&k)?, from(&v)?);
        }
        map.into()
    }
    // set
    else if let Ok(pset) = ob.downcast::<PySet>() {
        let mut set = BTreeSet::new();
        for v in pset {
            set.insert(from(&v)?);
        }
        set.into()
    }
    // frozen set
    else if let Ok(pfset) = ob.downcast::<PyFrozenSet>() {
        //
        let mut set = BTreeSet::new();
        for v in pfset {
            set.insert(from(&v)?);
        }
        set.into()
    }
    // lists and tuples
    else if let Ok(plist) = ob.downcast::<PyList>() {
        let mut array = Vec::new();
        for v in plist {
            array.push(from(&v)?);
        }
        array.into()
    } else if let Ok(ptuple) = ob.downcast::<PyTuple>() {
        let mut array = Vec::new();
        for v in ptuple {
            array.push(from(&v)?);
        }
        array.into()
    }
    // String
    else if let Ok(s) = ob.extract::<String>() {
        s.into()
    }
    // Numeric
    else if let Ok(v) = ob.extract::<i64>() {
        v.into()
    } else if let Ok(v) = ob.extract::<u64>() {
        v.into()
    } else if let Ok(v) = ob.extract::<f64>() {
        v.into()
    }
    // Boolean
    else if let Ok(b) = ob.extract::<bool>() {
        b.into()
    }
    // None
    else if ob.downcast::<PyNone>().is_ok() {
        Value::Null
    }
    // Anything that is a sequence
    else if let Ok(pseq) = ob.downcast::<PySequence>() {
        let mut array = Vec::new();
        for i in 0..pseq.len()? {
            array.push(from(&pseq.get_item(i)?)?);
        }
        array.into()
    }
    // Anything that is a map
    else if let Ok(pmap) = ob.downcast::<PyMapping>() {
        let mut map = BTreeMap::new();
        let keys = pmap.keys()?;
        let values = pmap.values()?;
        for i in 0..keys.len()? {
            let key = keys.get_item(i)?;
            let value = values.get_item(i)?;
            map.insert(from(&key)?, from(&value)?);
        }
        map.into()
    } else {
        return Err(PyErr::new::<PyTypeError, _>(
            "object cannot be converted to RegoValue",
        ));
    })
}

fn to(mut v: Value, py: Python<'_>) -> Result<PyObject> {
    Ok(match v {
        Value::Null => None::<u64>.to_object(py),

        // TODO: Revisit this mapping
        Value::Undefined => None::<u64>.to_object(py),

        Value::Bool(b) => b.to_object(py),
        Value::String(s) => s.to_object(py),

        Value::Number(_) => {
            if let Ok(f) = v.as_f64() {
                f.to_object(py)
            } else if let Ok(u) = v.as_u64() {
                u.to_object(py)
            } else {
                v.as_i64()?.to_object(py)
            }
        }

        Value::Array(_) => {
            let list = PyList::empty_bound(py);
            for v in std::mem::take(v.as_array_mut()?) {
                list.append(to(v, py)?)?;
            }
            list.into()
        }

        Value::Set(_) => {
            let set = PySet::empty_bound(py)?;
            for v in std::mem::take(v.as_set_mut()?) {
                set.add(to(v, py)?)?;
            }
            set.into()
        }

        Value::Object(_) => {
            let dict = PyDict::new_bound(py);
            for (k, v) in std::mem::take(v.as_object_mut()?) {
                dict.set_item(to(k, py)?, to(v, py)?)?;
            }
            dict.into()
        }
    })
}


#[pyclass(unsendable)]
pub struct PaplRegoEngine {
    engine: RegoEngine,
}

#[pymethods]
impl PaplRegoEngine {
    #[new]
    pub fn new() -> Self {
        PaplRegoEngine {
            engine: RegoEngine::new(false),
        }
    }

    pub fn get_package(&self) -> PyResult<Vec<String>>{
        let result = self.engine.get_packages();
        match result {
            Ok(packages) => Ok(packages),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string())),
        }
    }

    pub fn add_bundles(&mut self, bundles: &Bound<'_, PyList>) -> PyResult<()> {
        let bundles = bundles.extract::<Vec<String>>();

        match bundles {
            Ok(bundles) => {
                let _ = self.engine.add_bundles(&bundles);
                Ok(())
            },
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string())),
        }
    }

    pub fn add_input(&mut self,input: String) -> PyResult<()> {
        let result = self.engine.add_input(Some(input));
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string())),
        }
    }

    pub fn add_input_json(&mut self, input_json: String) -> PyResult<()> {
        let result = self.engine.add_input_json(Some(input_json));
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn add_policy_from_file(&mut self, file: String) -> PyResult<()> {
        let result = self.engine.add_policy_from_file(file);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn add_policy_from_string(&mut self,path: String, policy: String) -> PyResult<()> {
        let result = self.engine.add_policy_from_string(path,policy);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn close(&mut self) -> PyResult<()>{
        let _ = self.engine.close();
        Ok(())
    }

    pub fn add_data(&mut self, data: String) -> PyResult<()> {
        let result = self.engine.add_data(Some(data));
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn add_data_from_string(&mut self, data: String) -> PyResult<()> {
        let result = self.engine.add_data_from_string(data);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string()))
        }
    }

    pub fn clear_data(&mut self) {
        self.engine.clear_data();
    }

    pub fn eval_query(&mut self, query: String, py: Python<'_>) -> PyResult<PyObject>  {
        let results = self.engine.eval_query(query, false);

        if results.is_err() {
            return Err(PyErr::new::<PyException, _>(results.err().expect("error occur while evaluating query").to_string()));
        }

        let rlist = PyList::empty_bound(py);
        for result in results.unwrap().result.into_iter() {
            let rdict = PyDict::new_bound(py);

            let elist = PyList::empty_bound(py);
            for expr in result.expressions.into_iter() {
                let edict = PyDict::new_bound(py);
                edict.set_item("value".to_object(py), to(expr.value, py).unwrap())?;
                edict.set_item("text".to_object(py), expr.text.as_ref().to_object(py))?;

                let ldict = PyDict::new_bound(py);
                ldict.set_item("row".to_object(py), expr.location.row.to_object(py)).unwrap();
                ldict.set_item("col".to_object(py), expr.location.col.to_object(py)).unwrap();

                edict.set_item("location".to_object(py), ldict)?;
                elist.append(edict)?;
            }

            rdict.set_item("expressions".to_object(py), elist)?;
            rdict.set_item("bindings".to_object(py), to(result.bindings, py).unwrap())?;
            rlist.append(rdict)?;
        }
        let dict = PyDict::new_bound(py);
        dict.set_item("result".to_object(py), rlist)?;
        Ok(dict.into())
    }

    pub fn eval_rule(&mut self, path: String,py: Python<'_>) -> PyResult<PyObject> {
        let result = self.engine.eval_rule(path);
        match result {
            Ok(v) => {
                let v = to(v, py);
                match v {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<PyException, _>(e.to_string())),
                }
            },
            Err(e) => Err(PyErr::new::<PyException, _>(e.to_string())),
        }
    }
}


