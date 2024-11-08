use pyo3::prelude::*;

mod helper;
pub mod parallel;
pub mod serial;

#[pyfunction]
/// Generate a list of n random numbers with each number bounded by some number
fn list_bounded(len: usize, bound: usize) -> PyResult<Vec<usize>> {
    if len < helper::available_cores() * 1_000 {
        serial::list_bounded(len, bound)
    } else {
        parallel::list_bounded(len, bound)
    }
}

#[pyfunction]
/// Generate a list of n random numbers
fn list(len: usize) -> PyResult<Vec<usize>> {
    if len < helper::available_cores() * 1_000 {
        serial::list(len)
    } else {
        parallel::list(len)
    }
}

pub fn register(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "list")?;

    m.add_function(wrap_pyfunction!(list_bounded, m)?)?;
    m.add_function(wrap_pyfunction!(list, m)?)?;

    let parallel_submodule = PyModule::new(py, "parallel")?;
    parallel_submodule.add_function(wrap_pyfunction!(
        parallel::list_bounded,
        parallel_submodule
    )?)?;
    parallel_submodule.add_function(wrap_pyfunction!(parallel::list, parallel_submodule)?)?;
    m.add_submodule(parallel_submodule)?;

    let serial_submodule = PyModule::new(py, "serial")?;
    serial_submodule.add_function(wrap_pyfunction!(serial::list_bounded, serial_submodule)?)?;
    serial_submodule.add_function(wrap_pyfunction!(serial::list, serial_submodule)?)?;
    m.add_submodule(serial_submodule)?;

    Ok(m)
}
