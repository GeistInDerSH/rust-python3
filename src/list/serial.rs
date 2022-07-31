use pyo3::prelude::*;
use rand::Rng;

#[pyfunction]
pub fn list_bounded(len: usize, bound: usize) -> PyResult<Vec<usize>> {
    let mut rng = rand::thread_rng();

    let mut v = vec![0; len];
    for item in v.iter_mut().take(len) {
        *item = rng.gen_range(0..bound);
    }

    Ok(v)
}

#[pyfunction]
pub fn list(len: usize) -> PyResult<Vec<usize>> {
    let mut rng = rand::thread_rng();

    let mut v = vec![0; len];
    for item in v.iter_mut().take(len) {
        *item = rng.gen();
    }

    Ok(v)
}
