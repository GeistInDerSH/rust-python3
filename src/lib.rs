use pyo3::prelude::*;
use rand::Rng;

mod math;

#[pyfunction]
/// int_sum generates the sum of a vector of integers
fn int_sum(vec: Vec<usize>) -> PyResult<usize> {
    Ok(vec.iter().sum())
}

#[pyfunction]
/// Generate a list of n random numbers with each number bounded by some number
fn rand_list(len: usize, bound: usize) -> PyResult<Vec<usize>> {
    let mut rng = rand::thread_rng();
    let v = (0..len)
        .map(|_| {
            let n: usize = rng.gen();
            n % bound
        })
        .collect::<Vec<usize>>();
    Ok(v)
}

#[pymodule]
fn rpython(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(int_sum, m)?)?;
    m.add_function(wrap_pyfunction!(math::square_and_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(rand_list, m)?)?;
    Ok(())
}
