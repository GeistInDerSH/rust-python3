use pyo3::prelude::*;

mod math;

mod list;

#[pyfunction]
/// int_sum generates the sum of a vector of integers
fn int_sum(vec: Vec<usize>) -> PyResult<usize> {
    Ok(vec.iter().sum())
}

#[pymodule]
fn rpython(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(int_sum, m)?)?;
    m.add_function(wrap_pyfunction!(math::square_and_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(list::list_bounded, m)?)?;
    m.add_function(wrap_pyfunction!(list::list, m)?)?;

    let parallel_submodule = PyModule::new(_py, "parallel")?;
    parallel_submodule.add_function(wrap_pyfunction!(
        list::parallel::list_bounded,
        parallel_submodule
    )?)?;
    parallel_submodule.add_function(wrap_pyfunction!(list::parallel::list, parallel_submodule)?)?;

    let serial_submodule = PyModule::new(_py, "serial")?;
    serial_submodule.add_function(wrap_pyfunction!(
        list::serial::list_bounded,
        serial_submodule
    )?)?;
    serial_submodule.add_function(wrap_pyfunction!(list::serial::list, serial_submodule)?)?;

    m.add_submodule(parallel_submodule)?;
    m.add_submodule(serial_submodule)?;

    Ok(())
}
