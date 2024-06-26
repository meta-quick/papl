use pyo3::prelude::*;
use pyo3::types::*;
mod rego;
mod cedar;
mod store;

/// A Python module implemented in Rust.
#[pymodule]
fn paply(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_class::<rego::PaplRegoEngine>();
    let _ = m.add_class::<store::PaplStore>();
    let _ = m.add_class::<cedar::PaplCedarEngine>();
    Ok(())
}
