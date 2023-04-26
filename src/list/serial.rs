use pyo3::prelude::*;
use rand::Rng;

#[pyfunction(bound = 256)]
/// Generate a list of random numbers, with a set bound for the random numbers
pub fn list_bounded(len: usize, bound: usize) -> PyResult<Vec<usize>> {
    let mut rng = rand::thread_rng();
    Ok((0..len).map(|_| rng.gen_range(0..bound)).collect())
}

#[pyfunction]
/// Generate a list of random numbers, with no bound on the random numbers
pub fn list(len: usize) -> PyResult<Vec<usize>> {
    let mut rng = rand::thread_rng();
    Ok((0..len).map(|_| rng.gen()).collect())
}
