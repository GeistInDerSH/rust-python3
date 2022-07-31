use pyo3::prelude::*;

mod alloc;
mod math;

#[pyfunction]
/// int_sum generates the sum of a vector of integers
fn int_sum(vec: Vec<usize>) -> PyResult<usize> {
    Ok(vec.iter().sum())
}

#[pymodule]
fn rpython(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(int_sum, m)?)?;
    m.add_function(wrap_pyfunction!(math::square_and_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(alloc::rand_list, m)?)?;
    m.add_function(wrap_pyfunction!(alloc::serial_rand_list, m)?)?;
    m.add_function(wrap_pyfunction!(alloc::parallel_rand_list, m)?)?;
    Ok(())
}
