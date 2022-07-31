use pyo3::prelude::*;

pub mod parallel;
pub mod serial;

#[pyfunction]
/// Generate a list of n random numbers with each number bounded by some number
pub fn list_bounded(len: usize, bound: usize) -> PyResult<Vec<usize>> {
    if len < 250 {
        serial::list_bounded(len, bound)
    } else {
        parallel::list_bounded(len, bound)
    }
}

#[pyfunction]
/// Generate a list of n random numbers
pub fn list(len: usize) -> PyResult<Vec<usize>> {
    if len < 250 {
        serial::list(len)
    } else {
        parallel::list(len)
    }
}
